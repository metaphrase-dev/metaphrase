use errors::*;
use database;
use diesel::prelude::*;
use models::*;

pub mod middleware;

pub fn authenticate_user(email: &str, password: &str) -> Result<(User, Session), StringError> {
    let user = try!(retrieve_user(&email));

    try!(verify_password(&user, &password));

    let session = try!(create_session(&user));

    Ok((user, session))
}

pub fn authenticate_token(auth_token: &str) -> Result<Session, StringError> {
    use schema::sessions::dsl::*;
    use time;

    let connection = try!(database::establish_connection());

    match sessions.filter(token.eq(auth_token)).first::<Session>(&connection) {
        Ok(session) => {
            let session_expired_at = time::strptime(session.expired_at.as_str(), "%F %T").unwrap();

            if session_expired_at > time::now_utc() {
                Ok(session)
            } else {
                Err(StringError("Session expired"))
            }
        },
        Err(_) => Err(StringError("Session not found")),
    }
}

pub fn create_user(new_email: &str, new_password: &str) -> Result<User, StringError> {
    use diesel;
    use diesel::expression::dsl::sql;
    use schema::users::dsl::*;
    use schema::users::table;

    let new_hashed_password = try!(hash_password(&new_password));

    let new_user = NewUser {
        email: new_email.to_string(),
        hashed_password: new_hashed_password,
    };

    let connection = try!(database::establish_connection());

    diesel::insert(&new_user)
        .into(table)
        .execute(&connection)
        .expect("Error saving new user");

    let inserted_user = users.filter(id.eq(sql("last_insert_rowid()")))
        .get_result::<User>(&connection)
        .expect("Error getting inserted user");

    Ok(inserted_user)
}

pub fn delete_session(token_to_delete: String) -> Result<(), StringError> {
    use diesel;
    use schema::sessions::dsl::*;

    let connection = try!(database::establish_connection());

    let deleted = try!(
        diesel::delete(
            sessions.filter(
                token.eq(&token_to_delete)
            )
        ).execute(&connection)
    );

    match deleted {
        0 => Err(StringError("No session were deleted")),
        _ => Ok(())
    }
}

pub fn retrieve_user(user_email: &str) -> Result<User, StringError> {
    use schema::users::dsl::*;

    let connection = try!(database::establish_connection());

    match users.filter(email.eq(user_email)).first::<User>(&connection) {
        Ok(user) => Ok(user),
        Err(_) => Err(StringError("User not found")),
    }
}

fn create_session(user: &User) -> Result<Session, StringError> {
    use diesel;
    use diesel::expression::dsl::sql;
    use schema::sessions::dsl::*;
    use schema::sessions::table;
    use time::{Duration, now_utc, strftime};

    let connection = try!(database::establish_connection());

    let now = now_utc();
    let session_expired_at = now + Duration::days(7);

    let new_session = NewSession {
        token: try!(generate_token()),
        user_id: user.id,
        expired_at: try!(strftime("%F %T", &session_expired_at)),
    };

    try!(diesel::insert(&new_session).into(table).execute(&connection));

    let session = try!(sessions.filter(id.eq(sql("last_insert_rowid()"))).get_result::<Session>(&connection));

    Ok(session)
}

fn generate_token() -> Result<String, StringError> {
    use rand;
    use rand::Rng;

    let token = rand::thread_rng()
        .gen_ascii_chars()
        .take(64)
        .collect();

    Ok(token)
}

fn hash_password(password: &str) -> Result<String, StringError> {
    use pwhash::bcrypt;

    match bcrypt::hash(password) {
        Ok(password) => Ok(password),
        Err(_) => Err(StringError("Can’t hash password")),
    }
}

fn verify_password(user: &User, password: &str) -> Result<(), StringError> {
    use pwhash::bcrypt;

    if bcrypt::verify(password, user.hashed_password.as_str()) {
        Ok(())
    } else {
        Err(StringError("Authentication failed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticate_user_when_success() {
        use time;

        let email = "raphael@lustin.fr".to_string();
        let password = "testpassword".to_string();

        let result = authenticate_user(&email, &password);

        assert_eq!(false, result.is_err());

        let (user, session) = result.unwrap();

        assert_eq!(email, user.email);
        assert_eq!(user.id, session.user_id);
        assert_eq!(64, session.token.len());
        assert_eq!(1, session.user_id);

        let expired_at = time::strptime(session.expired_at.as_str(), "%F %T").unwrap();

        assert!(expired_at > time::now_utc());
    }

    #[test]
    fn test_authenticate_user_when_user_does_not_exist() {
        let email = "jean-michel@not-found.net".to_string();
        let password = "testpassword".to_string();

        let result = authenticate_user(&email, &password);

        assert!(result.is_err());
        assert_eq!(StringError("User not found"), result.err().unwrap())
    }

    #[test]
    fn test_authenticate_user_when_wrong_password() {
        let email = "raphael@lustin.fr".to_string();
        let password = "wrongpassword".to_string();

        let result = authenticate_user(&email, &password);

        assert!(result.is_err());
        assert_eq!(StringError("Authentication failed"), result.err().unwrap())
    }

    #[test]
    fn test_authenticate_token_when_success() {
        let token = "goodtokenfortests".to_string();

        let result = authenticate_token(&token);

        assert_eq!(false, result.is_err());

    }

    #[test]
    fn test_authenticate_token_when_expired() {
        let token = "expiredtokenfortests".to_string();

        let result = authenticate_token(&token);

        assert!(result.is_err());
        assert_eq!(StringError("Session expired"), result.err().unwrap())
    }

    #[test]
    fn test_authenticate_token_when_not_found() {
        let token = "not_found_token".to_string();

        let result = authenticate_token(&token);

        assert!(result.is_err());
        assert_eq!(StringError("Session not found"), result.err().unwrap())
    }

    #[test]
    fn test_create_user_when_success() {
        let email = "test@email.com".to_string();
        let password = "testpassword".to_string();

        let result = create_user(&email, &password);

        assert_eq!(false, result.is_err());

        let user = result.unwrap();

        assert_eq!(email, user.email);
        assert!(password != user.hashed_password);
    }

    #[test]
    fn test_delete_session_when_success() {
        let token = "tokentodelete".to_string();

        let result = delete_session(token);

        assert_eq!(false, result.is_err());
        assert_eq!((), result.unwrap());
    }

    #[test]
    fn test_delete_session_when_token_does_not_exist() {
        let token = "nonexistingtoken".to_string();

        let result = delete_session(token);

        assert!(result.is_err());
        assert_eq!(StringError("No session were deleted"), result.err().unwrap())
    }

    #[test]
    fn test_retrieve_user_when_success() {
        let email = "raphael@lustin.fr".to_string();

        let result = retrieve_user(&email);

        assert_eq!(false, result.is_err());
        assert_eq!(email, result.unwrap().email);
    }

    #[test]
    fn test_retrieve_user_when_not_found() {
        let email = "jean-michel@not-found.net".to_string();

        let result = retrieve_user(&email);

        assert!(result.is_err());
        assert_eq!(StringError("User not found"), result.err().unwrap())
    }
}

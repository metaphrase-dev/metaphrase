use errors::*;
use database;
use diesel::prelude::*;
use models::*;

pub mod middleware;

pub fn authenticate_user(email: &str, password: &str) -> Result<(User, Session), LughError> {
    let user = retrieve_user(&email)?;

    verify_password(&user, &password)?;

    let session = create_session(&user)?;

    Ok((user, session))
}

pub fn authenticate_token(auth_token: &str) -> Result<Session, LughError> {
    use schema::sessions::dsl::*;
    use time;

    let connection = database::establish_connection()?;

    match sessions.filter(token.eq(auth_token)).first::<Session>(&connection) {
        Ok(session) => {
            let session_expired_at = time::strptime(session.expired_at.as_str(), "%F %T").unwrap();

            if session_expired_at > time::now_utc() {
                Ok(session)
            } else {
                Err(LughError::Unauthorized("Session expired".to_string()))
            }
        },
        Err(_) => Err(LughError::Unauthorized("Session not found".to_string())),
    }
}

pub fn create_user(new_email: &str, new_password: &str) -> Result<User, LughError> {
    use diesel;
    use diesel::expression::dsl::sql;
    use schema::users::dsl::*;
    use schema::users::table;

    let new_hashed_password = hash_password(&new_password)?;

    let new_user = NewUser {
        email: new_email.to_string(),
        hashed_password: new_hashed_password,
    };

    let connection = database::establish_connection()?;

    diesel::insert(&new_user)
        .into(table)
        .execute(&connection)
        .expect("Error saving new user");

    let inserted_user = users.filter(id.eq(sql("last_insert_rowid()")))
        .get_result::<User>(&connection)
        .expect("Error getting inserted user");

    Ok(inserted_user)
}

pub fn delete_session(token_to_delete: String) -> Result<(), LughError> {
    use diesel;
    use schema::sessions::dsl::*;

    let connection = database::establish_connection()?;

    let deleted = diesel::delete(
        sessions.filter(
            token.eq(&token_to_delete)
        )
    ).execute(&connection)?;

    match deleted {
        0 => Err(LughError::NotFound("No session were deleted".to_string())),
        _ => Ok(())
    }
}

pub fn retrieve_user(user_email: &str) -> Result<User, LughError> {
    use schema::users::dsl::*;

    let connection = database::establish_connection()?;

    match users.filter(email.eq(user_email)).first::<User>(&connection) {
        Ok(user) => Ok(user),
        Err(_) => Err(LughError::NotFound(format!("User not found with email={}", user_email))),
    }
}

fn create_session(user: &User) -> Result<Session, LughError> {
    use diesel;
    use diesel::expression::dsl::sql;
    use schema::sessions::dsl::*;
    use schema::sessions::table;
    use time::{Duration, now_utc, strftime};

    let connection = database::establish_connection()?;

    let now = now_utc();
    let session_expired_at = now + Duration::days(7);

    let new_session = NewSession {
        token: generate_token()?,
        user_id: user.id,
        expired_at: strftime("%F %T", &session_expired_at)?,
    };

    diesel::insert(&new_session).into(table).execute(&connection)?;

    let session = sessions.filter(id.eq(sql("last_insert_rowid()"))).get_result::<Session>(&connection)?;

    Ok(session)
}

fn generate_token() -> Result<String, LughError> {
    use rand;
    use rand::Rng;

    let token = rand::thread_rng()
        .gen_ascii_chars()
        .take(64)
        .collect();

    Ok(token)
}

fn hash_password(password: &str) -> Result<String, LughError> {
    use pwhash::bcrypt;

    match bcrypt::hash(password) {
        Ok(password) => Ok(password),
        Err(_) => Err(LughError::Unauthorized("Can’t hash password".to_string())),
    }
}

fn verify_password(user: &User, password: &str) -> Result<(), LughError> {
    use pwhash::bcrypt;

    if bcrypt::verify(password, user.hashed_password.as_str()) {
        Ok(())
    } else {
        Err(LughError::Unauthorized(format!("Authentication failed for user with email={}", user.email)))
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
        assert_eq!(LughError::NotFound(format!("User not found with email={}", email)), result.err().unwrap())
    }

    #[test]
    fn test_authenticate_user_when_wrong_password() {
        let email = "raphael@lustin.fr".to_string();
        let password = "wrongpassword".to_string();

        let result = authenticate_user(&email, &password);

        assert!(result.is_err());
        assert_eq!(LughError::Unauthorized(format!("Authentication failed for user with email={}", email)), result.err().unwrap())
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
        assert_eq!(LughError::Unauthorized("Session expired".to_string()), result.err().unwrap())
    }

    #[test]
    fn test_authenticate_token_when_not_found() {
        let token = "not_found_token".to_string();

        let result = authenticate_token(&token);

        assert!(result.is_err());
        assert_eq!(LughError::Unauthorized("Session not found".to_string()), result.err().unwrap())
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
        assert_eq!(LughError::NotFound("No session were deleted".to_string()), result.err().unwrap())
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
        assert_eq!(LughError::NotFound(format!("User not found with email={}", email)), result.err().unwrap())
    }
}

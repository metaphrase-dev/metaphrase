use errors::*;
use iron::typemap;
use schema::*;

#[derive(RustcEncodable)]
pub struct NewSession {
    pub token: String,
    pub user_id: i32,
    pub expired_at: String,
}

Insertable! {
    (sessions)
    pub struct NewSession {
        pub token: String,
        pub user_id: i32,
        pub expired_at: String,
    }
}

pub struct NewTranslation {
    pub key: String,
    pub locale: String,
    pub content: String,
}

Insertable! {
    (translations)
    pub struct NewTranslation {
        pub key: String,
        pub locale: String,
        pub content: String,
    }
}

pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
}

Insertable! {
    (users)
    pub struct NewUser {
        pub email: String,
        pub hashed_password: String,
    }
}

pub struct Session {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub created_at: String,
    pub expired_at: String,
}

impl Session {
    pub fn user(&self) -> Result<User, StringError> {
        use database;
        use diesel::prelude::*;
        use schema::users::dsl::*;

        let connection = try!(database::establish_connection());

        match users.find(&self.user_id).first::<User>(&connection) {
            Ok(user) => Ok(user),
            Err(_) => Err(StringError("User not found for this Session")),
        }
    }
}

Queryable! {
    pub struct Session {
        pub id: i32,
        pub token: String,
        pub user_id: i32,
        pub created_at: String,
        pub expired_at: String,
    }
}

impl typemap::Key for Session { type Value = Session; }

#[derive(RustcEncodable)]
pub struct TranslationForLocale {
    pub id: i32,
    pub locale: String,
    pub content: Option<String>,
    pub created_at: String,
}

#[derive(RustcEncodable)]
pub struct Translation {
    pub id: i32,
    pub key: String,
    pub locale: String,
    pub content: Option<String>,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

Queryable! {
    pub struct Translation {
        pub id: i32,
        pub key: String,
        pub locale: String,
        pub content: Option<String>,
        pub created_at: String,
        pub deleted_at: Option<String>,
    }
}

pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub created_at: String,

}

Queryable! {
    pub struct User {
        pub id: i32,
        pub email: String,
        pub hashed_password: String,
        pub created_at: String,

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_session_when_not_found() {
        let result = session_for_user_id(999999).user();

        assert!(result.is_err());
        assert_eq!(StringError("User not found for this Session"), result.err().unwrap())
    }

    #[test]
    fn test_get_user_session_when_success() {
        let result = session_for_user_id(1).user();

        assert_eq!(false, result.is_err());
        assert_eq!("raphael@lustin.fr", result.unwrap().email)
    }

    fn session_for_user_id(user_id: i32) -> Session {
        use time::{now_utc, strftime};

        let now = strftime("%F %T", &now_utc()).unwrap();

        Session {
            id: 1,
            token: "secret_token".to_string(),
            user_id: user_id,
            created_at: now.clone(),
            expired_at: now,
        }
    }
}

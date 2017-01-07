use errors::*;
use iron::typemap;
use schema::*;

#[derive(Insertable, RustcEncodable)]
#[table_name="sessions"]
pub struct NewSession {
    pub token: String,
    pub user_id: i32,
    pub expired_at: String,
}

#[derive(Insertable)]
#[table_name="translations"]
pub struct NewTranslation {
    pub key: String,
    pub locale: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
}

#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub created_at: String,
    pub expired_at: String,
}

impl Session {
    pub fn user(&self) -> Result<User, LughError> {
        use database;
        use diesel::prelude::*;
        use schema::users::dsl::*;

        let connection = database::establish_connection()?;

        match users.find(&self.user_id).first::<User>(&connection) {
            Ok(user) => Ok(user),
            Err(_) => Err(LughError::NotFound("User not found for this Session".to_string())),
        }
    }
}

impl typemap::Key for Session { type Value = Session; }

#[derive(RustcEncodable)]
pub struct TranslationForLocale {
    pub id: i32,
    pub locale: String,
    pub content: Option<String>,
    pub created_at: String,
    pub user_id: Option<i32>,
    pub validator_id: Option<i32>,
    pub validated_at: Option<String>,
}

#[derive(AsChangeset, Queryable, RustcEncodable)]
pub struct Translation {
    pub id: i32,
    pub key: String,
    pub locale: String,
    pub content: Option<String>,
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub user_id: Option<i32>,
    pub validator_id: Option<i32>,
    pub validated_at: Option<String>,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
    pub created_at: String,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_session_when_not_found() {
        let result = session_for_user_id(999999).user();

        assert!(result.is_err());
        assert_eq!(LughError::NotFound("User not found for this Session".to_string()), result.err().unwrap())
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

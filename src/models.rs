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

Queryable! {
    pub struct Session {
        pub id: i32,
        pub token: String,
        pub user_id: i32,
        pub created_at: String,
        pub expired_at: String,
    }
}

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

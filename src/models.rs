use schema::translations;

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

#[derive(RustcEncodable)]
pub struct TranslationForLocale {
    pub id: i32,
    pub locale: String,
    pub content: Option<String>,
    pub timestamp: String,
}

#[derive(RustcEncodable)]
pub struct Translation {
    pub id: i32,
    pub key: String,
    pub locale: String,
    pub content: Option<String>,
    pub timestamp: String,
}

Queryable! {
    pub struct Translation {
        pub id: i32,
        pub key: String,
        pub locale: String,
        pub content: Option<String>,
        pub timestamp: String,
    }
}

#[derive(RustcEncodable)]
pub struct TranslationForLocale {
    pub id: i32,
    pub locale: String,
    pub content: String,
}

pub struct Translation {
    pub id: i32,
    pub key: String,
    pub locale: String,
    pub content: Option<String>,
}

Queryable! {
    pub struct Translation {
        pub id: i32,
        pub key: String,
        pub locale: String,
        pub content: Option<String>,
    }
}

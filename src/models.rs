#[derive(RustcEncodable)]
pub struct Translation {
    pub id: i32,
    pub key: String,
    pub locale: String,
    pub content: String,
}

Queryable! {
    pub struct Translation {
        pub id: i32,
        pub key: String,
        pub locale: String,
        pub content: String,
    }
}

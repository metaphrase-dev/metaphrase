#[cfg(test)]
mod tests {
    use super::super::common::*;

    use hyper::client::Response;
    use hyper::status::StatusCode;
    use rustc_serialize::json;
    use std::collections::HashMap;

    #[derive(RustcDecodable)]
    struct DeletedResult {
        deleted_translations: usize,
    }

    #[derive(RustcEncodable)]
    struct NewTranslation {
        key: Option<&'static str>,
        locale: Option<&'static str>,
        content: Option<&'static str>,
    }

    #[allow(dead_code)]
    #[derive(RustcDecodable)]
    struct TranslationForLocale {
        id: i32,
        locale: String,
        content: Option<String>,
        created_at: String,
    }

    #[derive(RustcDecodable)]
    pub struct Translation {
        pub id: i32,
        pub key: String,
        pub locale: String,
        pub content: Option<String>,
        pub created_at: String,
        pub deleted_at: Option<String>,
    }

    #[test]
    fn test_create_without_token() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: Some("en"),
            content: Some("I love train"),
        };

        let (response, content) = post_translation(new_translation, None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content)
    }

    #[test]
    fn test_create_without_key() {
        let new_translation = NewTranslation {
            key: None,
            locale: Some("en"),
            content: Some("I love train"),
        };

        let (response, _) = post_translation(new_translation, valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_locale() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: None,
            content: Some("I love train"),
        };

        let (response, _) = post_translation(new_translation, valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_content() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: Some("en"),
            content: None,
        };

        let (response, _) = post_translation(new_translation, valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_insert_and_delete() {
        // We fetch all translations
        let (response, content) = get("/api/v1/translations", valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_1 = parse_translations_by_locales(&content);

        assert_eq!(1, translations_1.len());
        assert_eq!(6, translations_1.get(&"ui.add".to_string()).unwrap().len());

        // We create new translations on key `test.hello`
        let (response, _) = post_translation(
            NewTranslation {
                key: Some("test.hello"),
                locale: Some("fr"),
                content: Some("Bonjour"),
            },
            valid_token()
        );

        assert_eq!(StatusCode::Created, response.status);

        let (response, _) = post_translation(
            NewTranslation {
                key: Some("test.hello"),
                locale: Some("en"),
                content: Some("Hello"),
            },
            valid_token()
        );

        assert_eq!(StatusCode::Created, response.status);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations", valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_2 = parse_translations_by_locales(&content);

        assert_eq!(2, translations_2.len());
        assert_eq!(6, translations_2.get(&"ui.add".to_string()).unwrap().len());
        assert_eq!(2, translations_2.get(&"test.hello".to_string()).unwrap().len());

        // We delete all translations with key equals to `test.hello`
        let (response, content) = delete("/api/v1/translations/test.hello", valid_token());
        let result: DeletedResult = json::decode(&content).unwrap();

        assert_eq!(StatusCode::Ok, response.status);
        assert_eq!(2, result.deleted_translations);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations", valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_3 = parse_translations_by_locales(&content);

        assert_eq!(1, translations_3.len());
        assert_eq!(6, translations_3.get(&"ui.add".to_string()).unwrap().len());

        // We fetch all translations with key `test.hello`
        let (response, content) = get("/api/v1/translations/test.hello", valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_4 = parse_translations(&content);

        assert_eq!(2, translations_4.len());
        assert_eq!("test.hello", translations_4[0].key);
        assert_eq!("test.hello", translations_4[1].key);
        assert!(translations_4[0].deleted_at.is_some());
        assert!(translations_4[1].deleted_at.is_some());
    }

    #[test]
    fn test_delete_without_token() {
        let (response, content) = delete("/api/v1/translations/hey.you", None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content);
    }

    #[test]
    fn test_delete_with_a_key_without_translations() {
        let (response, content) = delete("/api/v1/translations/not.found.key", valid_token());
        let result: DeletedResult = json::decode(&content).unwrap();

        assert_eq!(StatusCode::NotFound, response.status);
        assert_eq!(0, result.deleted_translations);
    }

    fn parse_translations_by_locales(ref content: &String) -> HashMap<String, Vec<TranslationForLocale>> {
        json::decode(&content).unwrap()
    }

    fn parse_translations(ref content: &String) -> Vec<Translation> {
        json::decode(&content).unwrap()
    }

    fn post_translation(translation: NewTranslation, token: Option<String>) -> (Response, String) {
        let body = json::encode(&translation).unwrap();

        post("/api/v1/translations", Some(body), token)
    }
}

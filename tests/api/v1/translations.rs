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

    #[derive(RustcDecodable)]
    pub struct LinterWarning {
        pub message: String,
        pub start: usize,
        pub end: usize,
    }

    #[derive(Clone, RustcDecodable)]
    struct TranslationForLocale {
        id: i32,
        locale: String,
        content: Option<String>,
        created_at: String,
        user_id: Option<i32>,
        validator_id: Option<i32>,
        validated_at: Option<String>,
    }

    #[derive(RustcDecodable)]
    struct CreateTranslationResponse {
        translation: Translation,
        warnings: Vec<LinterWarning>,
    }

    #[derive(RustcDecodable)]
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

    #[test]
    fn test_create_without_body() {
        let (response, content) = post("/api/v1/translations", &None, &valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
        assert_eq!("", content)
    }

    #[test]
    fn test_create_without_token() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: Some("en"),
            content: Some("I love train"),
        };

        let (response, content) = post_translation(&new_translation, &None);

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

        let (response, _) = post_translation(&new_translation, &valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_locale() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: None,
            content: Some("I love train"),
        };

        let (response, _) = post_translation(&new_translation, &valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_content() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: Some("en"),
            content: None,
        };

        let (response, _) = post_translation(&new_translation, &valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_insert_and_delete() {
        // We fetch all translations
        let (response, content) = get("/api/v1/translations", &valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_1 = parse_translations_by_locales(&content);

        assert_eq!(1, translations_1.len());
        assert_eq!(6, translations_1.get(&"ui.add".to_string()).unwrap().len());

        // We create new translations on key `test.hello`
        let (response, content) = post_translation(
            &NewTranslation {
                key: Some("test.hello"),
                locale: Some("fr"),
                content: Some("Bonjour"),
            },
            &valid_token()
        );

        assert_eq!(StatusCode::Created, response.status);

        let create_response = parse_create_translation(&content);

        assert_eq!("test.hello", create_response.translation.key);
        assert_eq!("fr", create_response.translation.locale);
        assert_eq!(Some("Bonjour".to_string()), create_response.translation.content);
        assert!(has_happened_now(&create_response.translation.created_at));
        assert_eq!(None, create_response.translation.deleted_at);
        assert_eq!(Some(1), create_response.translation.user_id);

        assert_eq!(true, create_response.warnings.is_empty());

        let (response, content) = post_translation(
            &NewTranslation {
                key: Some("test.hello"),
                locale: Some("en"),
                content: Some("Hello"),
            },
            &valid_token()
        );

        assert_eq!(StatusCode::Created, response.status);

        let create_response = parse_create_translation(&content);

        assert_eq!("test.hello", create_response.translation.key);
        assert_eq!("en", create_response.translation.locale);
        assert_eq!(Some("Hello".to_string()), create_response.translation.content);
        assert!(has_happened_now(&create_response.translation.created_at));
        assert_eq!(None, create_response.translation.deleted_at);
        assert_eq!(Some(1), create_response.translation.user_id);

        assert_eq!(true, create_response.warnings.is_empty());

        // We insert a translation with 2 linter warnings
        let (response, content) = post_translation(
            &NewTranslation {
                key: Some("test.me"),
                locale: Some("en"),
                content: Some("It's me..."),
            },
            &valid_token()
        );

        assert_eq!(StatusCode::Created, response.status);

        let create_response = parse_create_translation(&content);

        assert_eq!("test.me", create_response.translation.key);
        assert_eq!("en", create_response.translation.locale);
        assert_eq!(Some("It's me...".to_string()), create_response.translation.content);
        assert!(has_happened_now(&create_response.translation.created_at));
        assert_eq!(None, create_response.translation.deleted_at);
        assert_eq!(Some(1), create_response.translation.user_id);

        let warnings = create_response.warnings;

        assert_eq!(2, warnings.len());
        assert_eq!("Please use curly apostrophes.", warnings[0].message);
        assert_eq!(2, warnings[0].start);
        assert_eq!(3, warnings[0].end);

        assert_eq!("Please use the ellipsis symbol (`…`) instead of three dots (`...`).", warnings[1].message);
        assert_eq!(7, warnings[1].start);
        assert_eq!(10, warnings[1].end);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations", &valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_2 = parse_translations_by_locales(&content);

        assert_eq!(3, translations_2.len());
        assert_eq!(6, translations_2.get(&"ui.add".to_string()).unwrap().len());
        assert_eq!(2, translations_2.get(&"test.hello".to_string()).unwrap().len());
        assert_eq!(1, translations_2.get(&"test.me".to_string()).unwrap().len());

        // We delete all translations with key equals to `test.hello`
        let (response, content) = delete("/api/v1/translations/test.hello", &valid_token());
        let result: DeletedResult = json::decode(&content).unwrap();

        assert_eq!(StatusCode::Ok, response.status);
        assert_eq!(2, result.deleted_translations);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations", &valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_3 = parse_translations_by_locales(&content);

        assert_eq!(2, translations_3.len());
        assert_eq!(6, translations_3.get(&"ui.add".to_string()).unwrap().len());
        assert_eq!(1, translations_3.get(&"test.me".to_string()).unwrap().len());

        // We fetch all translations with key `test.hello`
        let (response, content) = get("/api/v1/translations/test.hello", &valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_4 = parse_translations(&content);

        assert_eq!(2, translations_4.len());
        assert_eq!("test.hello", translations_4[0].key);
        assert_eq!("test.hello", translations_4[1].key);
        assert!(translations_4[0].deleted_at.is_some());
        assert!(translations_4[1].deleted_at.is_some());
    }

    #[test]
    fn test_validate_without_token() {
        let (response, content) = post("/api/v1/translations/1/validate", &None, &None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content);
    }

    #[test]
    fn test_validate_when_not_found() {
        let (response, content) = post("/api/v1/translations/999999/validate", &None, &valid_token());

        assert_eq!(StatusCode::NotFound, response.status);
        assert_eq!("", content);
    }

    #[test]
    fn test_validate_with_success() {
        // We fetch all translations
        let (response, content) = get("/api/v1/translations", &valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_1 = parse_translations_by_locales(&content);

        for translation in &translations_1[&"ui.add".to_string()] {
            // None are validated
            assert_eq!(None, translation.validator_id);
        }

        // We validate the first translation
        let (response, content) = post("/api/v1/translations/1/validate", &None, &valid_token());

        assert_eq!(StatusCode::NoContent, response.status);
        assert_eq!("", content);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations", &valid_token());

        assert_eq!(StatusCode::Ok, response.status);

        let translations_2 = parse_translations_by_locales(&content);
        let ui_add_translations = &translations_2[&"ui.add".to_string()];

        // The first translation is validated
        let validated_translation = ui_add_translations[0].clone();
        assert_eq!(1, validated_translation.id);
        assert_eq!(Some(1), validated_translation.validator_id);
        assert!(has_happened_now(validated_translation.validated_at.unwrap().as_str()));

        // The other translations are not validated
        assert_eq!(None, ui_add_translations[1].validator_id);
        assert_eq!(None, ui_add_translations[1].validated_at);
        assert_eq!(None, ui_add_translations[2].validator_id);
        assert_eq!(None, ui_add_translations[2].validated_at);
        assert_eq!(None, ui_add_translations[3].validator_id);
        assert_eq!(None, ui_add_translations[3].validated_at);
        assert_eq!(None, ui_add_translations[4].validator_id);
        assert_eq!(None, ui_add_translations[4].validated_at);
        assert_eq!(None, ui_add_translations[5].validator_id);
    }

    #[test]
    fn test_delete_without_token() {
        let (response, content) = delete("/api/v1/translations/hey.you", &None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content);
    }

    #[test]
    fn test_delete_with_a_key_without_translations() {
        let (response, content) = delete("/api/v1/translations/not.found.key", &valid_token());
        let result: DeletedResult = json::decode(&content).unwrap();

        assert_eq!(StatusCode::NotFound, response.status);
        assert_eq!(0, result.deleted_translations);
    }

    fn parse_translations_by_locales(content: &str) -> HashMap<String, Vec<TranslationForLocale>> {
        json::decode(content).unwrap()
    }

    fn parse_create_translation(content: &str) -> CreateTranslationResponse {
        json::decode(content).unwrap()
    }

    fn parse_translations(content: &str) -> Vec<Translation> {
        json::decode(content).unwrap()
    }

    fn post_translation(translation: &NewTranslation, token: &Option<String>) -> (Response, String) {
        let body = json::encode(&translation).unwrap();

        post("/api/v1/translations", &Some(body), token)
    }
}

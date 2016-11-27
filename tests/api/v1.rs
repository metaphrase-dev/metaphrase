#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use hyper::*;
    use hyper::client::Response;
    use hyper::header::Headers;
    use hyper::status::StatusCode;
    use rustc_serialize::json;
    use std::collections::HashMap;
    use std::env;
    use std::io::Read;

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

    #[test]
    fn test_index() {
        let (response, result) = get("/api/v1");

        assert_eq!(response.status, Ok);
        assert_eq!(result, "Welcome to Lugh API!");
    }

    #[test]
    fn test_create_without_key() {
        let new_translation = NewTranslation {
            key: None,
            locale: Some("en"),
            content: Some("I love train"),
        };

        let (response, _) = post_translation(new_translation);

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_locale() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: None,
            content: Some("I love train"),
        };

        let (response, _) = post_translation(new_translation);

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_content() {
        let new_translation = NewTranslation {
            key: Some("test.i_love_train"),
            locale: Some("en"),
            content: None,
        };

        let (response, _) = post_translation(new_translation);

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_insert_and_delete() {
        // We fetch all translations
        let (response, content) = get("/api/v1/translations");

        assert_eq!(Ok, response.status);

        let translations_1 = parse_translations(&content);

        assert_eq!(1, translations_1.len());
        assert_eq!(6, translations_1.get(&"ui.add".to_string()).unwrap().len());

        // We create new translations on key `test.hello`
        let (response, _) = post_translation(
            NewTranslation {
                key: Some("test.hello"),
                locale: Some("fr"),
                content: Some("Bonjour"),
            }
        );

        assert_eq!(StatusCode::Created, response.status);

        let (response, _) = post_translation(
            NewTranslation {
                key: Some("test.hello"),
                locale: Some("en"),
                content: Some("Hello"),
            }
        );

        assert_eq!(StatusCode::Created, response.status);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations");

        assert_eq!(Ok, response.status);

        let translations_2 = parse_translations(&content);

        assert_eq!(2, translations_2.len());
        assert_eq!(6, translations_2.get(&"ui.add".to_string()).unwrap().len());
        assert_eq!(2, translations_2.get(&"test.hello".to_string()).unwrap().len());

        // We delete all translations with key equals to `test.hello`
        let (response, _) = delete_translations("test.hello");

        assert_eq!(StatusCode::NoContent, response.status);

        // We fetch all translations
        let (response, content) = get("/api/v1/translations");

        assert_eq!(Ok, response.status);

        let translations_3 = parse_translations(&content);

        assert_eq!(1, translations_3.len());
        assert_eq!(6, translations_3.get(&"ui.add".to_string()).unwrap().len());
    }

    #[test]
    fn test_delete_without_key() {
        let (response, _) = delete("/api/v1/translations", "{}".to_string());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    fn application_json_headers() -> Headers {
        use hyper::header::{Headers, ContentType};
        use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

        let mut headers = Headers::new();
        headers.set(
            ContentType(
                Mime(
                    TopLevel::Application,
                    SubLevel::Json,
                    vec![(Attr::Charset, Value::Utf8)]
                )
            )
        );

        headers
    }

    fn delete(path: &'static str, body: String) -> (Response, String) {
        let mut response = Client::new()
            .delete(&url(path))
            .headers(application_json_headers())
            .body(&body)
            .send()
            .unwrap();

        let mut content = String::new();
        response.read_to_string(&mut content).unwrap();

        (response, content)
    }

    fn delete_translations(key_to_delete: &'static str) -> (Response, String) {
        #[derive(RustcEncodable)]
        struct Body { key: &'static str }

        let body = json::encode(&Body { key: key_to_delete }).unwrap();

        delete("/api/v1/translations", body)
    }

    fn get(path: &'static str) -> (Response, String) {
        let mut response = Client::new()
            .get(&url(path))
            .send()
            .unwrap();

        let mut result = String::new();
        response.read_to_string(&mut result).unwrap();

        (response, result)
    }

    fn parse_translations(ref content: &String) -> HashMap<String, Vec<TranslationForLocale>> {
        json::decode(&content).unwrap()
    }

    fn post(path: &'static str, body: String) -> (Response, String) {
        let mut response = Client::new()
            .post(&url(path))
            .headers(application_json_headers())
            .body(&body)
            .send()
            .unwrap();

        let mut content = String::new();
        response.read_to_string(&mut content).unwrap();

        (response, content)
    }

    fn post_translation(translation: NewTranslation) -> (Response, String) {
        let body = json::encode(&translation).unwrap();

        post("/api/v1/translations", body)
    }

    fn url(path: &'static str) -> String {
        dotenv().ok();

        let hostname = env::var("LUGH_BIND")
            .expect("LUGH_BIND must be set");

        "http://".to_string() + hostname.as_str() + path
    }
}

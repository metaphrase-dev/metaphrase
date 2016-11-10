#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use hyper::*;
    use hyper::client::Response;
    use rustc_serialize::json;
    use std::collections::HashMap;
    use std::env;
    use std::io::Read;

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
        let mut response = get_response("/api/v1");

        let mut result = String::new();
        response.read_to_string(&mut result).unwrap();

        assert_eq!(response.status, Ok);
        assert_eq!(result, "Welcome to Lugh API!");
    }

    #[test]
    fn test_translations_index() {
        let mut response = get_response("/api/v1/translations");

        let mut result = String::new();
        response.read_to_string(&mut result).unwrap();

        assert_eq!(Ok, response.status);

        let translations: HashMap<String, Vec<TranslationForLocale>> = json::decode(&result).unwrap();
        let ui_add_translations = translations.get(&"ui.add".to_string()).unwrap();

        assert_eq!(1, translations.len());
        assert_eq!(6, ui_add_translations.len());
    }

    fn get_response(path: &'static str) -> Response {
        Client::new()
            .get(&url(path))
            .send()
            .unwrap()
    }

    fn url(path: &'static str) -> String {
        dotenv().ok();

        let hostname = env::var("LUGH_BIND")
            .expect("LUGH_BIND must be set");

        "http://".to_string() + hostname.as_str() + path
    }
}

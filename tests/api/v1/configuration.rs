#[cfg(test)]
mod tests {
    use super::super::common::*;

    use hyper::status::StatusCode;
    use rustc_serialize::json;

    #[derive(RustcDecodable)]
    struct Configuration {
        locales: Vec<String>,
    }

    #[test]
    fn test_get_configuration_without_token() {
        let (response, content) = get("/api/v1/configuration", None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content)
    }

    #[test]
    fn test_get_configuration_with_success() {
        let (response, content) = get("/api/v1/configuration", valid_token());
        let configuration = parse_configuration(&content);

        assert_eq!(StatusCode::Ok, response.status);
        assert_eq!(5, configuration.locales.len());
    }

    fn parse_configuration(content: &str) -> Configuration {
        json::decode(content).unwrap()
    }
}

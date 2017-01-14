mod common;
mod configuration;
mod sessions;
mod translations;
mod users;

#[cfg(test)]
mod tests {
    use super::common::*;

    use hyper::status::StatusCode;

    #[test]
    fn test_index() {
        let (response, result) = get("/api/v1", valid_token());

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(result, "Welcome to Lugh API!");
    }

    #[test]
    fn test_index_without_token() {
        let (response, result) = get("/api/v1", None);

        assert_eq!(response.status, StatusCode::Unauthorized);
        assert_eq!(result, "");
    }

    #[test]
    fn test_index_with_a_bad_token() {
        let (response, result) = get("/api/v1", Some("badtoken".to_string()));

        assert_eq!(response.status, StatusCode::Unauthorized);
        assert_eq!(result, "");
    }

    #[test]
    fn test_index_without_content_type() {
        use hyper::*;
        use hyper::header::{Authorization, Bearer, Headers};
        use std::io::Read;

        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: valid_token().unwrap() }));

        let mut response = Client::new()
            .get(&url("/api/v1"))
            .headers(headers)
            .send()
            .unwrap();

        let mut result = String::new();
        response.read_to_string(&mut result).unwrap();

        assert_eq!(StatusCode::BadRequest, response.status);
        assert_eq!("", result);
    }
}

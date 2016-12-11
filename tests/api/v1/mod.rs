mod common;
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
}

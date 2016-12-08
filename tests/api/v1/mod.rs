mod common;
mod translations;

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
}

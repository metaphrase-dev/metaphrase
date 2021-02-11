mod common;
mod configuration;
mod sessions;
mod translations;
mod users;

#[cfg(test)]
mod tests {
    use super::common::*;

    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn test_index() {
        let (response, result) = get("/api/v1/", valid_token()).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(result, "Welcome to Lugh API!");
    }

    #[actix_rt::test]
    async fn test_index_without_token() {
        let (response, result) = get("/api/v1", None).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
        assert_eq!(result, "");
    }

    #[actix_rt::test]
    async fn test_index_with_a_bad_token() {
        let badtoken = "badtoken".to_string();
        let (response, result) = get("/api/v1", Some(badtoken)).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
        assert_eq!(result, "");
    }

    /*#[test]
    fn test_index_without_content_type() {
        use hyper::header::{Authorization, Bearer, Headers};
        use hyper::*;

        let mut headers = Headers::new();
        headers.set(Authorization(Bearer {
            token: valid_token().unwrap(),
        }));

        let mut response = Client::new()
            .get(&url("/api/v1"))
            .headers(headers)
            .send()
            .unwrap();

        let mut result = String::new();
        response.read_to_string(&mut result).unwrap();

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
        assert_eq!("", result);
    }*/
}

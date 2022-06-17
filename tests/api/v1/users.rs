#[cfg(test)]
mod tests {
    use super::super::common::*;

    use actix_web::{dev::ServiceResponse, http::StatusCode};

    use serde_json;

    #[derive(Serialize)]
    struct NewUser {
        email: Option<&'static str>,
        password: Option<&'static str>,
    }

    #[actix_rt::test]
    async fn test_create_without_body() {
        let response = post("/api/v1/users", None, valid_token()).await;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[actix_rt::test]
    async fn test_create_without_token() {
        let new_user = NewUser {
            email: Some("user@domain.com"),
            password: Some("p4ssw0rd"),
        };

        let response = post_user(&new_user, None).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    #[actix_rt::test]
    async fn test_create_without_email() {
        let new_user = NewUser {
            email: None,
            password: Some("p4ssw0rd"),
        };

        let response = post_user(&new_user, valid_token()).await;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[actix_rt::test]
    async fn test_create_without_password() {
        let new_user = NewUser {
            email: Some("user@domain.com"),
            password: None,
        };

        let response = post_user(&new_user, valid_token()).await;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[actix_rt::test]
    async fn test_create_with_success() {
        let new_user = NewUser {
            email: Some("r@l.fr"),
            password: Some("p4ssw0rd"),
        };

        let response = post_user(&new_user, valid_token()).await;

        assert_eq!(StatusCode::CREATED, response.status());
    }

    async fn post_user(user: &NewUser, token: Option<String>) -> ServiceResponse {
        let body = serde_json::to_string(&user).unwrap();

        post("/api/v1/users", Some(body), token).await
    }
}

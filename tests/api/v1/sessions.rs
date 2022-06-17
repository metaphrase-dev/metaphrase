#[cfg(test)]
mod tests {
    use super::super::common::*;

    use actix_web::{dev::ServiceResponse, http::StatusCode};

    use serde_json;
    use time::{format_description::well_known, OffsetDateTime, PrimitiveDateTime};

    #[derive(Serialize)]
    struct LoginParams {
        email: Option<&'static str>,
        password: Option<&'static str>,
    }

    #[derive(Deserialize)]
    struct Session {
        token: String,
        user_id: i32,
        expired_at: String,
    }

    #[actix_rt::test]
    async fn test_login_without_body() {
        let response = post("/api/v1/login", None, None).await;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[actix_rt::test]
    async fn test_login_without_email() {
        let login_params = LoginParams {
            email: None,
            password: Some("testpassword"),
        };

        let response = login(&login_params).await;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[actix_rt::test]
    async fn test_login_without_password() {
        let login_params = LoginParams {
            email: Some("raphael@lustin.fr"),
            password: None,
        };

        let response = login(&login_params).await;

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }

    #[actix_rt::test]
    async fn test_login_with_wrong_password() {
        let login_params = LoginParams {
            email: Some("raphael@lustin.fr"),
            password: Some("wrongpassword"),
        };

        let response = login(&login_params).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }

    #[actix_rt::test]
    async fn test_login_then_logout_with_success() {
        use time;

        let login_params = LoginParams {
            email: Some("raphael@lustin.fr"),
            password: Some("testpassword"),
        };

        let response = login(&login_params).await;

        assert_eq!(StatusCode::CREATED, response.status());

        let session: Session = serde_json::from_str(&body_as_string(response).await).unwrap();

        assert_eq!(64, session.token.len());
        assert_eq!(1, session.user_id);

        let expired_at: PrimitiveDateTime =
            time::PrimitiveDateTime::parse(session.expired_at.as_str(), &well_known::Rfc3339)
                .unwrap();
        assert!(expired_at.assume_utc() > OffsetDateTime::now_utc());

        let response = post("/api/v1/logout", None, Some(session.token.clone())).await;

        assert_eq!(StatusCode::NO_CONTENT, response.status());
        assert_eq!("", body_as_string(response).await);

        // We check that we are disconnected
        let response = post("/api/v1/logout", None, Some(session.token)).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
        assert_eq!("", body_as_string(response).await);
    }

    #[actix_rt::test]
    async fn test_logout_without_token() {
        let response = post("/api/v1/logout", None, None).await;

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
        assert_eq!("", body_as_string(response).await);
    }

    async fn login(params: &LoginParams) -> ServiceResponse {
        let body = serde_json::to_string(&params).unwrap();

        post("/api/v1/login", Some(body), None).await
    }
}

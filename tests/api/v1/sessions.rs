#[cfg(test)]
mod tests {
    use super::super::common::*;

    use hyper::client::Response;
    use hyper::status::StatusCode;
    use rustc_serialize::json;

    #[derive(RustcEncodable)]
    struct LoginParams {
        email: Option<&'static str>,
        password: Option<&'static str>,
    }

    #[derive(RustcDecodable)]
    struct Session {
        token: String,
        user_id: i32,
        expired_at: String,
    }

    #[test]
    fn test_login_without_body() {
        let (response, content) = post("/api/v1/login", &None, &None);

        assert_eq!(StatusCode::BadRequest, response.status);
        assert_eq!("", content)
    }

    #[test]
    fn test_login_without_email() {
        let login_params = LoginParams {
            email: None,
            password: Some("testpassword"),
        };

        let (response, _) = login(&login_params);

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_login_without_password() {
        let login_params = LoginParams {
            email: Some("raphael@lustin.fr"),
            password: None,
        };

        let (response, _) = login(&login_params);

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_login_with_wrong_password() {
        let login_params = LoginParams {
            email: Some("raphael@lustin.fr"),
            password: Some("wrongpassword"),
        };

        let (response, _) = login(&login_params);

        assert_eq!(StatusCode::Unauthorized, response.status);
    }

    #[test]
    fn test_login_then_logout_with_success() {
        use time;

        let login_params = LoginParams {
            email: Some("raphael@lustin.fr"),
            password: Some("testpassword"),
        };

        let (response, content) = login(&login_params);

        assert_eq!(StatusCode::Created, response.status);

        let session: Session = json::decode(&content).unwrap();

        assert_eq!(64, session.token.len());
        assert_eq!(1, session.user_id);

        let expired_at = time::strptime(session.expired_at.as_str(), "%F %T").unwrap();
        assert!(expired_at > time::now_utc());

        let (response, content) = post("/api/v1/logout", &None, &Some(session.token.clone()));

        assert_eq!(StatusCode::NoContent, response.status);
        assert_eq!("", content);

        // We check that we are disconnected
        let (response, content) = post("/api/v1/logout", &None, &Some(session.token));

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content);
    }

    #[test]
    fn test_logout_without_token() {
        let (response, content) = post("/api/v1/logout", &None, &None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content);
    }

    fn login(params: &LoginParams) -> (Response, String) {
        let body = json::encode(&params).unwrap();

        post("/api/v1/login", &Some(body), &None)
    }
}

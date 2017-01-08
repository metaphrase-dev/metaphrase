#[cfg(test)]
mod tests {
    use super::super::common::*;

    use hyper::client::Response;
    use hyper::status::StatusCode;
    use rustc_serialize::json;

    #[derive(RustcEncodable)]
    struct NewUser {
        email: Option<&'static str>,
        password: Option<&'static str>,
    }

    #[test]
    fn test_create_without_body() {
        let (response, content) = post("/api/v1/users", None, valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
        assert_eq!("", content)
    }

    #[test]
    fn test_create_without_token() {
        let new_user = NewUser {
            email: Some("user@domain.com"),
            password: Some("p4ssw0rd"),
        };

        let (response, content) = post_user(new_user, None);

        assert_eq!(StatusCode::Unauthorized, response.status);
        assert_eq!("", content);
    }

    #[test]
    fn test_create_without_email() {
        let new_user = NewUser {
            email: None,
            password: Some("p4ssw0rd"),
        };

        let (response, _) = post_user(new_user, valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_without_password() {
        let new_user = NewUser {
            email: Some("user@domain.com"),
            password: None,
        };

        let (response, _) = post_user(new_user, valid_token());

        assert_eq!(StatusCode::BadRequest, response.status);
    }

    #[test]
    fn test_create_with_success() {
        let new_user = NewUser {
            email: Some("r@l.fr"),
            password: Some("p4ssw0rd"),
        };

        let (response, _) = post_user(new_user, valid_token());

        assert_eq!(StatusCode::Created, response.status);
    }

    fn post_user(user: NewUser, token: Option<String>) -> (Response, String) {
        let body = json::encode(&user).unwrap();

        post("/api/v1/users", Some(body), token)
    }
}

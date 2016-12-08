extern crate iron;

use iron::prelude::*;
use iron::BeforeMiddleware;

use super::*;

pub struct AuthenticationMiddleware;

impl AuthenticationMiddleware {
    fn authorize(&self, token: &String) -> bool {
        match authenticate_token(token) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn unauthorized_error(&self) -> Result<(), IronError> {
        use iron::status;
        use errors::*;

        Err(IronError {
            error: Box::new(StringError("Unauthorized")),
            response: Response::with(status::Unauthorized),
        })
    }
}

impl BeforeMiddleware for AuthenticationMiddleware {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        use iron::headers::*;

        if request.url.path().last() == Some(&"login") {
            return Ok(())
        }

        match request.headers.get::<Authorization<Bearer>>() {
            Some(&Authorization(Bearer { ref token })) => {
                if self.authorize(token) {
                    Ok(())
                } else {
                    self.unauthorized_error()
                }
            }
            None => self.unauthorized_error()
        }
    }
}

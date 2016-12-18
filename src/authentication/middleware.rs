extern crate iron;

use iron::prelude::*;
use iron::BeforeMiddleware;

use super::*;

pub struct AuthenticationMiddleware;

impl AuthenticationMiddleware {
    fn authorize(&self, token: &str) -> Result<Session, StringError> {
        authenticate_token(token)
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
                match self.authorize(token) {
                    Ok(session) => {
                        request.extensions.insert::<Session>(session);
                        Ok(())
                    },
                    Err(_) => {
                        self.unauthorized_error()
                    },
                }
            },
            None => self.unauthorized_error()
        }
    }
}

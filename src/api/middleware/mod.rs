extern crate iron;

use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::mime::{Mime, SubLevel, TopLevel};

pub struct ContentTypeMiddleware;

impl ContentTypeMiddleware {
    fn bad_request_error(&self) -> Result<(), IronError> {
        use iron::status;
        use errors::LughError;

        Err(IronError {
            error: Box::new(
              LughError::BadRequest("`Content-Type` header must be set to `application/json`".to_string())
            ),
            response: Response::with(status::BadRequest),
        })
    }
}

impl BeforeMiddleware for ContentTypeMiddleware {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        use iron::headers::*;

        match request.headers.get::<ContentType>() {
            Some(&ContentType(Mime(ref top_level, ref sub_level, _))) => {
                match (top_level, sub_level) {
                    (&TopLevel::Application, &SubLevel::Json) => Ok(()),
                    (_, _) => self.bad_request_error(),
                }
            },
            None => self.bad_request_error(),
        }
    }
}

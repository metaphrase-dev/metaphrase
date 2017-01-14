use iron::prelude::*;
use iron::status;
use router::Router;

use errors::LughError;

pub mod common;
pub mod configuration;
pub mod router;
pub mod sessions;
pub mod translations;
pub mod users;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to Lugh API!")))
}

pub fn router() -> Result<Router, LughError> {
    router::router()
}

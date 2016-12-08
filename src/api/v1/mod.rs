use iron::prelude::*;
use iron::status;

pub mod common;
pub mod sessions;
pub mod translations;
pub mod users;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to Lugh API!")))
}

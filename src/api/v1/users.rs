use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;

use authentication;
use super::common::*;

pub fn create(request: &mut Request) -> IronResult<Response> {
    let email = try!(get_param(request, "email"));
    let password = try!(get_param(request, "password"));

    let inserted_user = try!(authentication::create_user(&email, &password));

    println!("User saved with id={}", inserted_user.id);

    Ok(Response::with((ContentType::json().0, status::Created)))
}

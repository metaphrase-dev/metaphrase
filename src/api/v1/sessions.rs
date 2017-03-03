use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

use authentication::{authenticate_user, delete_session};
use super::common::*;

pub fn login(request: &mut Request) -> IronResult<Response> {
    use models::NewSession;

    let email = get_param(request, "email")?;
    let password = get_param(request, "password")?;

    let (user, session) = authenticate_user(&email, &password)?;

    let new_session = NewSession {
        token: session.token,
        user_id: user.id,
        expired_at: session.expired_at,
    };

    let payload = json::encode(&new_session).unwrap();

    Ok(Response::with((ContentType::json().0, status::Created, payload)))
}

pub fn logout(request: &mut Request) -> IronResult<Response> {
    use models::Session;

    let current_session = request.extensions.get::<Session>().unwrap();

    delete_session(current_session.token.as_str())?;

    Ok(Response::with((ContentType::json().0, status::NoContent)))
}

use errors::LughError;
use iron::prelude::*;
use models::*;
use params::{Params, Value};

pub fn current_user(request: &Request) -> Result<User, LughError> {
    use models::Session;

    let current_session = request.extensions.get::<Session>().unwrap();

    current_session.user()
}

pub fn now_str() -> Result<String, LughError> {
    use time::{now_utc, strftime};

    match strftime("%F %T", &now_utc()) {
        Ok(time) => Ok(time),
        Err(_) => Err(LughError::ParseFailed("Time parse error".to_string())),
    }
}

pub fn get_param(request: &mut Request, name: &str) -> Result<String, LughError> {
    let parameters = request.get_ref::<Params>().unwrap();

    match parameters.find(&[name]) {
        Some(&Value::String(ref value)) => Ok(value.clone()),
        _ => Err(LughError::BadRequest(format!("You must provide a {}", name))),
    }
}

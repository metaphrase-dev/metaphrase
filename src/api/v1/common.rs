use errors::BadRequestError;
use iron::prelude::*;
use params::{Params, Value};

pub fn get_param(request: &mut Request, name: &str) -> Result<String, BadRequestError> {
    let parameters = request.get_ref::<Params>().unwrap();

    match parameters.find(&[name]) {
        Some(&Value::String(ref value)) => Ok(value.clone()),
        _ => Err(BadRequestError(format!("You must provide a {}", name))),
    }
}

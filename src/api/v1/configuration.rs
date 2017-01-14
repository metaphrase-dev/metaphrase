use dotenv::dotenv;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use std::env;

#[derive(RustcEncodable)]
struct Configuration {
    locales: Vec<String>,
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    dotenv().ok();

    let locales: Vec<String> = env::var("LUGH_AVAILABLE_LOCALES")
        .unwrap()
        .split(' ')
        .map(|x| x.to_owned())
        .collect();

    let configuration = Configuration {
        locales: locales,
    };

    let payload = json::encode(&configuration).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

use hyper::*;
use hyper::client::Response;
use hyper::header::Headers;
use std::io::Read;

pub fn delete(path: &'static str, body: String) -> (Response, String) {
    let mut response = Client::new()
        .delete(&url(path))
        .headers(application_json_headers())
        .body(&body)
        .send()
        .unwrap();

    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();

    (response, content)
}

pub fn get(path: &'static str) -> (Response, String) {
    let mut response = Client::new()
        .get(&url(path))
        .send()
        .unwrap();

    let mut result = String::new();
    response.read_to_string(&mut result).unwrap();

    (response, result)
}

pub fn post(path: &'static str, body: String) -> (Response, String) {
    let mut response = Client::new()
        .post(&url(path))
        .headers(application_json_headers())
        .body(&body)
        .send()
        .unwrap();

    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();

    (response, content)
}

fn application_json_headers() -> Headers {
    use hyper::header::{Headers, ContentType};
    use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

    let mut headers = Headers::new();
    headers.set(
        ContentType(
            Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)]
            )
        )
    );

    headers
}

fn url(path: &'static str) -> String {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let hostname = env::var("LUGH_BIND")
        .expect("LUGH_BIND must be set");

    "http://".to_string() + hostname.as_str() + path
}

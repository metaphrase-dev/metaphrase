use hyper::*;
use hyper::client::Response;
use hyper::header::Headers;
use std::io::Read;

pub fn delete(path: &'static str, token: Option<String>) -> (Response, String) {
    let mut response = Client::new()
        .delete(&url(path))
        .headers(headers(token))
        .send()
        .unwrap();

    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();

    (response, content)
}

pub fn get(path: &'static str, token: Option<String>) -> (Response, String) {
    let mut response = Client::new()
        .get(&url(path))
        .headers(headers(token))
        .send()
        .unwrap();

    let mut result = String::new();
    response.read_to_string(&mut result).unwrap();

    (response, result)
}

pub fn post(path: &'static str, body: Option<String>, token: Option<String>) -> (Response, String) {
    let client = Client::new();

    let mut request = client.post(&url(path))
        .headers(headers(token));

    request = match body {
        Some(ref body) => request.body(body),
        None => request,
    };

    let mut response = request.send().unwrap();

    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();

    (response, content)
}

pub fn valid_token() -> Option<String> {
    Some("goodtokenfortests".to_string())
}

fn headers(token: Option<String>) -> Headers {
    use hyper::header::{Authorization, Bearer, ContentType, Headers};
    use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

    let mut headers = Headers::new();

    match token {
        Some(token) => headers.set(Authorization(Bearer { token: token })),
        None => {},
    };

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

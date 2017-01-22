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

pub fn url(path: &'static str) -> String {
    use std::env;

    let hostname = env::var("LUGH_BIND")
        .expect("LUGH_BIND must be set");

    "http://".to_string() + hostname.as_str() + path
}

pub fn valid_token() -> Option<String> {
    Some("goodtokenfortests".to_string())
}

pub fn has_happened_now(time_str: &str) -> bool {
    use time::{Duration, now_utc, strptime};

    let time = strptime(time_str, "%F %T").unwrap();
    let now = now_utc();
    let min = now - Duration::seconds(2);

    time > min && time <= now
}

fn headers(token: Option<String>) -> Headers {
    use hyper::header::{Authorization, Bearer, ContentType, Headers};
    use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

    let mut headers = Headers::new();

    if let Some(token) = token {
        headers.set(Authorization(Bearer { token: token }))
    }

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

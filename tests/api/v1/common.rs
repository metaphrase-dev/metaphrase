use actix_web::{
    dev::{Body, ResponseBody, ServiceResponse},
    http::{self, Method},
};
use actix_web::{test, web, App};
use test::TestRequest;
use time::PrimitiveDateTime;

use crate::*;
use lugh;

trait BodyTest {
    fn as_str(&self) -> &str;
}

impl BodyTest for ResponseBody<Body> {
    fn as_str(&self) -> &str {
        match self {
            ResponseBody::Body(ref b) => match b {
                Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                Body::Empty => "",
                _ => panic!(format!("can't read the body?? {:#?}", b)),
            },
            ResponseBody::Other(ref b) => match b {
                Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                Body::Empty => "",
                _ => panic!(format!("can't read the body?? {:#?}", b)),
            },
        }
    }
}

pub async fn call_server(
    method: actix_web::http::Method,
    path: &'static str,
    body: Option<String>,
    token: Option<String>,
) -> ServiceResponse<Body> {
    let mut app = test::init_service(
        App::new()
            .wrap(lugh::logger::RequestLogger)
            .wrap(lugh::authentication::middleware::Authentication)
            .service(web::scope("/api/v1").configure(lugh::api::v1::config)),
    )
    .await;

    let simple_request = TestRequest::default().method(method).uri(path);
    let with_token = add_token_header(simple_request, &token);
    let with_body = add_body(with_token, &body);

    let req = with_body.to_request();
    test::call_service(&mut app, req).await
}

pub async fn delete(path: &'static str, token: Option<String>) -> (ServiceResponse, String) {
    let response: ServiceResponse = call_server(Method::DELETE, path, None, token).await;
    let body = response.response().body().as_str().to_string();

    (response, body)
}

pub async fn get(path: &'static str, token: Option<String>) -> (ServiceResponse, String) {
    let response: ServiceResponse = call_server(Method::GET, path, None, token).await;
    let body = response.response().body().as_str().to_string();

    (response, body)
}

pub async fn post(
    path: &'static str,
    body: Option<String>,
    token: Option<String>,
) -> (ServiceResponse, String) {
    let response: ServiceResponse = call_server(Method::POST, path, body, token).await;
    let response_body = response.response().body().as_str().to_string();

    (response, response_body)
}

pub fn valid_token() -> Option<String> {
    Some("goodtokenfortests".to_string())
}

pub fn has_happened_now(time_str: &str) -> bool {
    use time::{Duration, OffsetDateTime};

    let time: PrimitiveDateTime = time::parse(time_str, "%F %T").unwrap();
    let time_utc = time.assume_utc();
    let now = OffsetDateTime::now_utc();
    let min = now - Duration::seconds(2);

    time_utc > min && time_utc <= now
}

fn add_token_header(req: TestRequest, token: &Option<String>) -> TestRequest {
    if let Some(token) = token {
        req.header(http::header::AUTHORIZATION, format!("Bearer {}", token))
    } else {
        req
    }
}

fn add_body(req: TestRequest, body: &Option<String>) -> TestRequest {
    if let Some(body) = body {
        req.set_payload(body.clone())
            .header(http::header::CONTENT_TYPE, "application/json")
    } else {
        req.header(http::header::CONTENT_TYPE, "application/json")
    }
}

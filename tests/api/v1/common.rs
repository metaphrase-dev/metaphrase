use actix_web::{
    body::to_bytes,
    dev::ServiceResponse,
    http::{self, Method},
    web::Bytes,
};
use actix_web::{test, web, App};
use test::TestRequest;
use time::{format_description::well_known, PrimitiveDateTime};

use crate::*;
use metaphrase;

trait BodyTest {
    fn to_string(&self) -> String;
}

impl BodyTest for Bytes {
    fn to_string(&self) -> String {
        std::str::from_utf8(self).unwrap().to_string()
    }
}

pub async fn call_server(
    method: actix_web::http::Method,
    path: &'static str,
    body: Option<String>,
    token: Option<String>,
) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(metaphrase::logger::RequestLogger)
            .wrap(metaphrase::authentication::middleware::Authentication)
            .service(web::scope("/api/v1").configure(metaphrase::api::v1::config)),
    )
    .await;

    let simple_request = TestRequest::default().method(method).uri(path);
    let with_token = add_token_header(simple_request, &token);
    let with_body = add_body(with_token, &body);

    let req = with_body.to_request();
    test::call_service(&mut app, req).await
}

pub async fn delete(path: &'static str, token: Option<String>) -> ServiceResponse {
    call_server(Method::DELETE, path, None, token).await
}

pub async fn get(path: &'static str, token: Option<String>) -> ServiceResponse {
    call_server(Method::GET, path, None, token).await
}

pub async fn post(
    path: &'static str,
    body: Option<String>,
    token: Option<String>,
) -> ServiceResponse {
    call_server(Method::POST, path, body, token).await
}

pub fn valid_token() -> Option<String> {
    Some("goodtokenfortests".to_string())
}

pub fn has_happened_now(time_str: &str) -> bool {
    use time::{Duration, OffsetDateTime};

    let time: PrimitiveDateTime =
        time::PrimitiveDateTime::parse(time_str, &well_known::Rfc3339).unwrap();
    let time_utc = time.assume_utc();
    let now = OffsetDateTime::now_utc();
    let min = now - Duration::seconds(2);

    time_utc > min && time_utc <= now
}

fn add_token_header(req: TestRequest, token: &Option<String>) -> TestRequest {
    if let Some(token) = token {
        req.insert_header((http::header::AUTHORIZATION, format!("Bearer {}", token)))
    } else {
        req
    }
}

fn add_body(req: TestRequest, body: &Option<String>) -> TestRequest {
    if let Some(body) = body {
        req.set_payload(body.clone())
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
    } else {
        req.insert_header((http::header::CONTENT_TYPE, "application/json"))
    }
}

pub async fn body_as_string(response: ServiceResponse) -> String {
    to_bytes(response.into_body()).await.unwrap().to_string()
}

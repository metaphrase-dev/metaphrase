use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, http, Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, Either, Ready};

use crate::models::Session;

use super::authenticate_token;
pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

type ResultReady<SR, SE> = Ready<Result<SR, SE>>;

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, ResultReady<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if req.path().ends_with("login") {
            return Either::Left(self.service.call(req));
        }

        if let Some(raw_header_value) = req.headers().get(http::header::AUTHORIZATION) {
            if let Ok(header_value) = raw_header_value.to_str() {
                let token_string = header_value.to_string().replace("Bearer ", "");

                match authenticate_token(token_string.as_str()) {
                    Ok(session) => {
                        req.extensions_mut().insert::<Session>(session);

                        return Either::Left(self.service.call(req));
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }
        }

        Either::Right(ok(
            req.into_response(HttpResponse::Unauthorized().finish().into_body())
        ))
    }
}

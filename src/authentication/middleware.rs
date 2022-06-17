use std::rc::Rc;

use actix_service::{Service, Transform};
use actix_web::{
    body::BoxBody, dev::forward_ready, dev::ServiceRequest, dev::ServiceResponse, http, Error,
    HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};

use crate::models::Session;

use super::authenticate_token;
pub struct Authentication;

impl<S> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
    S: 'static,
{
    type Response = S::Response;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S: 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            if req.path().ends_with("login") {
                let resp = service.call(req).await?;

                return Ok(resp.map_into_boxed_body());
            }

            if let Some(raw_header_value) = req.headers().get(http::header::AUTHORIZATION) {
                if let Ok(header_value) = raw_header_value.to_str() {
                    let token_string = header_value.to_string().replace("Bearer ", "");

                    match authenticate_token(token_string.as_str()) {
                        Ok(session) => {
                            req.extensions_mut().insert::<Session>(session);

                            let resp = service.call(req).await?;

                            return Ok(resp.map_into_boxed_body());
                        }
                        Err(error) => {
                            eprintln!("{}", error);
                        }
                    }
                }
            }

            let resp = HttpResponse::Unauthorized().finish();

            Ok(req.into_response(resp).map_into_boxed_body())
        })
    }
}

extern crate time;

use std::convert::TryInto;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures_util::future::Future;
use futures_util::future::{ok, Ready};
use time::format_description::well_known;
use time::OffsetDateTime;

fn precise_time_ns() -> u64 {
    (OffsetDateTime::now_utc() - OffsetDateTime::UNIX_EPOCH)
        .whole_nanoseconds()
        .try_into()
        .unwrap_or(0)
}

pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware { service })
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

type BoxedFutureOutput<SR, SE> = Box<dyn Future<Output = Result<SR, SE>>>;

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<BoxedFutureOutput<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = precise_time_ns();

        info!(
            "Started {} \"{}\" for {:?} at {}",
            req.head().method,
            req.path(),
            match req.peer_addr() {
                Some(ip) => format!("{}", ip),
                None => "".to_string(),
            },
            time::OffsetDateTime::now_utc()
                .format(&well_known::Rfc3339)
                .unwrap()
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            let delta = (precise_time_ns() - start_time) as i64;
            let elapsed_ms = if delta <= 0 {
                0
            } else {
                ((delta as f64) / 1000000.0).round() as u64
            };

            info!("{} ({} ms)", res.status(), elapsed_ms);

            Ok(res)
        })
    }
}

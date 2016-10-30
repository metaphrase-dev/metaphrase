extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

pub struct RequestLogger;

impl typemap::Key for RequestLogger { type Value = u64; }

impl BeforeMiddleware for RequestLogger {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        request.extensions.insert::<RequestLogger>(precise_time_ns());
        println!(
            "Started {} \"{}\" for {}",
            request.method,
            format!("/{}", request.url.path().join("/")),
            request.remote_addr
        );

        Ok(())
    }
}

impl AfterMiddleware for RequestLogger {
    fn after(&self, request: &mut Request, response: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *request.extensions.get::<RequestLogger>().unwrap();
        println!("Request took {} ms", (delta as f64) / 1000000.0);

        Ok(response)
    }
}

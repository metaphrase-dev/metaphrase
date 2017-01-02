extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

pub struct RequestLogger;

impl RequestLogger {
    fn start(&self, request: &mut Request) {
        request.extensions.insert::<RequestLogger>(precise_time_ns());
    }

    fn elapsed_time(&self, request: &mut Request) -> Result<u64, IronError> {
        let start_time = *request.extensions.get::<RequestLogger>().unwrap();
        let delta = (precise_time_ns() - start_time) as i64;

        if delta <= 0 {
            Ok(0)
        } else {
            Ok(((delta as f64) / 1000000.0).round() as u64)
        }
    }
}

impl typemap::Key for RequestLogger { type Value = u64; }

impl BeforeMiddleware for RequestLogger {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        self.start(request);

        println!(
            "Started {} \"{}\" for {} at {}",
            request.method,
            format!("/{}", request.url.path().join("/")),
            request.remote_addr,
            time::now().strftime("%+").unwrap()
        );

        Ok(())
    }
}

impl AfterMiddleware for RequestLogger {
    fn after(&self, request: &mut Request, response: Response) -> IronResult<Response> {
        println!("{} ({} ms)", response.status.unwrap(), self.elapsed_time(request)?);

        Ok(response)
    }

    fn catch(&self, request: &mut Request, err: IronError) -> IronResult<Response> {
        println!(
            "{}: {} ({} ms)",
            err.response.status.unwrap(),
            err.error,
            self.elapsed_time(request)?
        );

        Err(err)
    }
}

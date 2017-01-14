#![feature(custom_attribute, custom_derive)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate iron;
#[macro_use] extern crate log;
extern crate mount;
extern crate params;
extern crate pwhash;
extern crate rand;
extern crate router;
extern crate rustc_serialize;
extern crate simplelog;
extern crate staticfile;
extern crate time;

use dotenv::dotenv;
use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use simplelog::{Config, TermLogger, LogLevelFilter};
use std::env;
use std::path::Path;

mod api;
mod authentication;
mod database;
mod errors;
mod schema;
mod logger;
mod models;

fn main() {
    dotenv().ok();

    TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    let router = api::v1::router().unwrap();

    let mut api_chain = Chain::new(router);
    api_chain.link_before(authentication::middleware::AuthenticationMiddleware);
    api_chain.link_before(api::middleware::ContentTypeMiddleware);

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("src/frontend/public/")));
    mount.mount("/api/v1", api_chain);

    let mut chain = Chain::new(mount);
    chain.link_before(logger::RequestLogger);
    chain.link_after(logger::RequestLogger);

    let bind = env::var("LUGH_BIND").unwrap_or("localhost:3000".to_string());
    info!("Start application on {}", bind);

    Iron::new(chain).http(bind.as_str())
        .expect("Can’t start application");
}

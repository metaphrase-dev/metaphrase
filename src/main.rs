#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate iron;
#[macro_use] extern crate log;
extern crate mount;
extern crate params;
extern crate pwhash;
extern crate rand;
extern crate regex;
extern crate router;
extern crate rustc_serialize;
extern crate simplelog;
extern crate staticfile;
extern crate time;
extern crate typographic_linter;

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
    check_environment_variables();

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

    let bind = env::var("LUGH_BIND").unwrap_or_else(|_| "localhost:3000".to_string());
    info!("Start application on {}", bind);

    Iron::new(chain).http(bind.as_str())
        .expect("Can’t start application");
}

fn check_environment_variables() {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

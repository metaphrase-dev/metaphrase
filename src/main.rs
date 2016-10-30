#![feature(custom_attribute, custom_derive, proc_macro)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate staticfile;
extern crate router;
extern crate rustc_serialize;
extern crate time;

use iron::prelude::*;
use staticfile::Static;
use router::Router;
use std::path::Path;

mod api;
mod database;
mod schema;
mod logger;
mod models;

fn main() {
    let mut router = Router::new();

    router.get("/", Static::new(Path::new("src/frontend/")), "index");
    router.get("/api", api::index, "api");
    router.get("/api/translations", api::translations, "translations");

    let mut chain = Chain::new(router);
    chain.link_before(logger::RequestLogger);
    chain.link_after(logger::RequestLogger);

    Iron::new(chain).http("localhost:3000").unwrap();
}

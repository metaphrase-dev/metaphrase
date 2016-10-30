#![feature(custom_attribute)]
#![feature(custom_derive)]
#![feature(proc_macro)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate staticfile;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use staticfile::Static;
use router::Router;
use std::path::Path;

mod api;
mod database;
mod schema;
mod models;

fn main() {
    let mut router = Router::new();

    router.get("/", Static::new(Path::new("src/frontend/")), "index");
    router.get("/api", api::index, "api");
    router.get("/api/translations", api::translations, "translations");

    Iron::new(router).http("localhost:3000").unwrap();
}

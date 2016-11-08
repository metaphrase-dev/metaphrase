#![feature(custom_attribute, custom_derive, proc_macro)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate staticfile;
extern crate router;
extern crate mount;
extern crate params;
extern crate rustc_serialize;
extern crate time;

use iron::prelude::*;
use staticfile::Static;
use router::Router;
use mount::Mount;
use std::path::Path;

mod api;
mod database;
mod schema;
mod logger;
mod models;

fn main() {
    let mut router = Router::new();
    router.get("/", api::v1::index, "api");
    router.get("/translations", api::v1::translations_index, "translations_index");
    router.post("/translations", api::v1::translations_create, "translations_create");

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("src/frontend/public/")));
    mount.mount("/api/v1", router);

    let mut chain = Chain::new(mount);
    chain.link_before(logger::RequestLogger);
    chain.link_after(logger::RequestLogger);

    Iron::new(chain).http("localhost:3000").unwrap();
}

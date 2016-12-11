#![feature(custom_attribute, custom_derive, proc_macro)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate mount;
extern crate params;
extern crate pwhash;
extern crate rand;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;
extern crate time;

use dotenv::dotenv;
use iron::prelude::*;
use staticfile::Static;
use router::Router;
use mount::Mount;
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

    let mut router = Router::new();
    router.get("/", api::v1::index, "api");

    router.post("/login", api::v1::sessions::login, "login");
    router.post("/logout", api::v1::sessions::logout, "logout");

    router.get("/translations", api::v1::translations::index, "translations_index");
    router.post("/translations", api::v1::translations::create, "translations_create");
    router.delete("/translations/:key", api::v1::translations::delete, "translations_delete");

    router.post("/users", api::v1::users::create, "users_create");

    let mut api_chain = Chain::new(router);
    api_chain.link_before(authentication::middleware::AuthenticationMiddleware);

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("src/frontend/public/")));
    mount.mount("/api/v1", api_chain);

    let mut chain = Chain::new(mount);
    chain.link_before(logger::RequestLogger);
    chain.link_after(logger::RequestLogger);

    let bind = env::var("LUGH_BIND").unwrap_or("localhost:3000".to_string());
    println!("Start application on {}", bind);

    Iron::new(chain).http(bind.as_str())
        .expect("Can’t start application");
}

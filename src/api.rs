extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rustc_serialize::json;
use std::env;

pub fn index(request: &mut Request) -> IronResult<Response> {
    println!("Started GET \"/api\" for {}", request.remote_addr);

    Ok(Response::with((status::Ok, "Welcome to Lugh API!")))
}

pub fn translations(request: &mut Request) -> IronResult<Response> {
    use schema::translations::dsl::*;
    use models::*;

    println!("Started GET \"/api/translations\" for {}", request.remote_addr);

    let connection = establish_connection();
    let results = translations.load::<Translation>(&connection)
        .expect("Error loading translations");

    println!("Returns {} translations", results.len());

    let payload = json::encode(&results).unwrap();

    Ok(Response::with((status::Ok, payload)))
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

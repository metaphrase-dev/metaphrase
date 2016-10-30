extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use diesel::prelude::*;
use rustc_serialize::json;
use database;

pub fn index(request: &mut Request) -> IronResult<Response> {
    println!("Started GET \"/api\" for {}", request.remote_addr);

    Ok(Response::with((status::Ok, "Welcome to Lugh API!")))
}

pub fn translations(request: &mut Request) -> IronResult<Response> {
    use schema::translations::dsl::*;
    use models::*;

    println!("Started GET \"/api/translations\" for {}", request.remote_addr);

    let connection = database::establish_connection();
    let results = translations.load::<Translation>(&connection)
        .expect("Error loading translations");

    println!("Returns {} translations", results.len());

    let payload = json::encode(&results).unwrap();

    Ok(Response::with((status::Ok, payload)))
}

extern crate iron;
extern crate rustc_serialize;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use diesel::prelude::*;
use rustc_serialize::json;
use database;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to Lugh API!")))
}

pub fn translations(_: &mut Request) -> IronResult<Response> {
    use diesel::expression::dsl::sql;
    use models::*;
    use schema::translations::dsl::*;

    let connection = database::establish_connection();
    let results = translations.filter(sql("id IN (SELECT MAX(id) FROM translations GROUP BY key, locale)"))
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    println!("Returns {} translations", results.len());

    let payload = json::encode(&results).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

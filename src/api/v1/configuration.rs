use diesel::prelude::*;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use std::collections::HashMap;

use database;
use models::*;
use serde_json;
use schema::settings::dsl::*;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let connection = database::establish_connection()?;

    let results = settings.load::<Setting>(&connection)
        .expect("Error loading settings");

    let mut configuration = HashMap::new();

    for setting in &results {
        let mut settings_for_key = configuration.entry(&setting.key)
            .or_insert_with(Vec::<String>::new);

        settings_for_key.push(setting.value.clone());
    }

    let payload = serde_json::to_string(&configuration).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

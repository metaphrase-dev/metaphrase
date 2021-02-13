use diesel::prelude::*;
use std::collections::HashMap;

use crate::models::*;
use crate::schema::settings::dsl::*;
use crate::{database, errors::MetaphraseError};

use actix_web::web;
use actix_web::Responder;

pub async fn index() -> Result<impl Responder, MetaphraseError> {
    let connection = database::establish_connection()?;

    let results = settings
        .load::<Setting>(&connection)
        .expect("Error loading settings");

    let mut configuration = HashMap::new();

    for setting in &results {
        let settings_for_key = configuration
            .entry(setting.key.clone())
            .or_insert_with(Vec::<String>::new);

        settings_for_key.push(setting.value.clone());
    }

    Ok(web::Json(configuration))
}

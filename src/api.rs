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
    use std::collections::HashMap;

    let connection = database::establish_connection();
    let results = translations.filter(
            sql("(key, locale, timestamp) IN (
                    SELECT key, locale, MAX(timestamp)
                    FROM translations
                    GROUP BY key, locale)")
        )
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    let mut all_translations = HashMap::new();

    for translation in &results {
      let mut translations_for_key = all_translations.entry(&translation.key)
          .or_insert(HashMap::<String, TranslationForLocale>::new());

      translations_for_key.insert(
          translation.locale.clone(),
          TranslationForLocale {
              id: translation.id,
              locale: translation.locale.clone(),
              content: translation.content.clone(),
              timestamp: translation.timestamp.clone(),
          }
      );
    }

    println!("Returns {} translations", all_translations.len());

    let payload = json::encode(&all_translations).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

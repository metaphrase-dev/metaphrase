extern crate iron;
extern crate rustc_serialize;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use diesel::expression::dsl::sql;
use diesel::prelude::*;
use models::*;
use rustc_serialize::json;
use schema::translations::dsl::*;
use database;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to Lugh API!")))
}

pub fn translations_index(_: &mut Request) -> IronResult<Response> {
    use std::collections::HashMap;

    let connection = database::establish_connection();
    let results = translations.filter(
            sql("key || locale || created_at IN (
                  SELECT key || locale || MAX(created_at)
                  FROM translations
                  GROUP BY key, locale)")
        )
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    let mut all_translations = HashMap::new();

    for translation in &results {
      let mut translations_for_key = all_translations.entry(&translation.key)
          .or_insert(Vec::<TranslationForLocale>::new());

      translations_for_key.push(
          TranslationForLocale {
              id: translation.id,
              locale: translation.locale.clone(),
              content: translation.content.clone(),
              created_at: translation.created_at.clone(),
          }
      );
    }

    println!("Returns {} translations", all_translations.len());

    let payload = json::encode(&all_translations).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

pub fn translations_create(request: &mut Request) -> IronResult<Response> {
    use diesel;
    use params::{Params, Value};
    use schema::translations;

    let parameters = request.get_ref::<Params>().unwrap();

    let new_key = match parameters.find(&["key"]) {
        Some(&Value::String(ref new_key)) => new_key,
        _ => panic!("Unexpected parameter type for key!"),
    };

    let new_locale = match parameters.find(&["locale"]) {
        Some(&Value::String(ref new_locale)) => new_locale,
        _ => panic!("Unexpected parameter type for locale!"),
    };

    let new_content = match parameters.find(&["content"]) {
        Some(&Value::String(ref new_content)) => new_content,
        _ => panic!("Unexpected parameter type for content!"),
    };

    let new_translation = NewTranslation {
        key: new_key.to_string(),
        locale: new_locale.to_string(),
        content: new_content.to_string(),
    };

    let connection = database::establish_connection();

    diesel::insert(&new_translation)
        .into(translations::table)
        .execute(&connection)
        .expect("Error saving new translation");

    let inserted_translation = translations.filter(id.eq(sql("last_insert_rowid()")))
        .get_result::<Translation>(&connection)
        .expect("Error getting inserted translation");

    println!("Translation saved with id={}", inserted_translation.id);

    let payload = json::encode(&inserted_translation).unwrap();

    Ok(Response::with((ContentType::json().0, status::Created, payload)))
}

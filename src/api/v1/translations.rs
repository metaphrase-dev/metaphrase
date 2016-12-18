use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use diesel::expression::dsl::sql;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use errors::*;
use models::*;
use rustc_serialize::json;
use schema::translations::dsl::*;

use database;
use super::common::*;

pub fn index(_: &mut Request) -> IronResult<Response> {
    use std::collections::HashMap;

    let connection = try!(database::establish_connection());
    let results = translations.filter(
            sql("key || locale || created_at IN (
                  SELECT key || locale || MAX(created_at)
                  FROM translations
                  WHERE deleted_at IS NULL
                  GROUP BY key, locale)")
        )
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    let mut all_translations = HashMap::new();

    for translation in &results {
      let mut translations_for_key = all_translations.entry(&translation.key)
          .or_insert_with(Vec::<TranslationForLocale>::new);

      translations_for_key.push(
          TranslationForLocale {
              id: translation.id,
              locale: translation.locale.clone(),
              content: translation.content.clone(),
              created_at: translation.created_at.clone(),
              user_id: translation.user_id,
              validator_id: translation.validator_id,
              validated_at: translation.validated_at.clone(),
          }
      );
    }

    println!("Returns {} translations", all_translations.len());

    let payload = json::encode(&all_translations).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    use diesel;
    use schema::translations;

    let new_key = try!(get_param(request, "key"));
    let new_locale = try!(get_param(request, "locale"));
    let new_content = try!(get_param(request, "content"));

    let user = current_user(request)?;

    let new_translation = NewTranslation {
        key: new_key,
        locale: new_locale,
        content: new_content,
        user_id: user.id,
    };

    let connection = try!(database::establish_connection());

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

pub fn show(request: &mut Request) -> IronResult<Response> {
    use router::Router;

    let translation_key = request.extensions.get::<Router>().unwrap().find("key").unwrap();

    let connection = try!(database::establish_connection());

    let all_translations = translations.filter(key.eq(translation_key))
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    println!("Returns {} translations", all_translations.len());

    let payload = json::encode(&all_translations).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

pub fn validate(request: &mut Request) -> IronResult<Response> {
    use diesel;
    use diesel::prelude::*;
    use router::Router;
    use schema::translations::dsl::*;

    let translation_id: i32 = request.extensions
        .get::<Router>().unwrap()
        .find("id").unwrap()
        .parse().unwrap();

    let user = current_user(request)?;

    let connection = database::establish_connection()?;
    let mut translation = find_translation(&connection, translation_id)?;

    translation.validator_id = Some(user.id);
    translation.validated_at = Some(now_str()?);

    let validated = diesel::update(translations.find(translation.id))
        .set(&translation)
        .execute(&connection)
        .expect(&format!("Unable to validate translation with id={}", &translation_id));

    let status = match validated {
        1 => status::NoContent,
        _ => status::InternalServerError,
    };

    Ok(Response::with((ContentType::json().0, status)))
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
    use diesel;
    use router::Router;

    let key_to_delete = request.extensions.get::<Router>().unwrap().find("key").unwrap();

    let connection = try!(database::establish_connection());

    let now = now_str()?;

    let selected_translations = translations.filter(key.eq(&key_to_delete))
        .filter(deleted_at.is_null());

    let deleted = diesel::update(selected_translations)
        .set(deleted_at.eq(now))
        .execute(&connection)
        .expect(&format!("Unable to delete translations with key={}", &key_to_delete));

    #[derive(RustcEncodable)]
    struct DeletedResult {
        deleted_translations: usize,
    };

    let payload = json::encode(&DeletedResult { deleted_translations: deleted }).unwrap();

    let status = match deleted {
        0 => status::NotFound,
        _ => status::Ok,
    };

    Ok(Response::with((ContentType::json().0, status, payload)))
}

fn find_translation(connection: &SqliteConnection, translation_id: i32) -> Result<Translation, NotFoundError> {
    use diesel::prelude::*;
    use schema::translations::dsl::*;

    match translations.find(translation_id).first::<Translation>(connection) {
        Ok(translation) => Ok(translation),
        Err(_) => Err(NotFoundError(format!("Can’t find Translation with id={}", translation_id))),
    }
}

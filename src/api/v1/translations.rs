use crate::errors::*;
use crate::models::*;
use crate::schema::translations::dsl::*;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use diesel::expression::dsl::sql;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::common::*;
use crate::database;

pub async fn index() -> Result<impl Responder, LughError> {
    use std::collections::HashMap;

    let connection = database::establish_connection()?;
    let results = translations
        .filter(sql("key || locale || created_at IN (
                  SELECT key || locale || MAX(created_at)
                  FROM translations
                  WHERE deleted_at IS NULL
                  GROUP BY key, locale)"))
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    let mut all_translations = HashMap::new();

    for translation in results {
        let translations_for_key = all_translations
            .entry(translation.key.clone())
            .or_insert_with(Vec::<TranslationForLocale>::new);

        translations_for_key.push(TranslationForLocale {
            id: translation.id,
            locale: translation.locale.clone(),
            content: translation.content.clone(),
            created_at: translation.created_at.clone(),
            user_id: translation.user_id,
            validator_id: translation.validator_id,
            validated_at: translation.validated_at.clone(),
        });
    }

    debug!("Returns {} translations", all_translations.len());

    Ok(web::Json(all_translations))
}

#[derive(Deserialize)]
pub struct CreateFormData {
    key: String,
    locale: String,
    content: String,
}

pub async fn create(
    form: web::Json<CreateFormData>,
    req: HttpRequest,
) -> Result<impl Responder, LughError> {
    use crate::schema::translations;
    use typographic_linter::errors::LinterWarning;
    use typographic_linter::Linter;

    #[derive(Serialize)]
    struct CreateTranslationResponse {
        translation: Translation,
        warnings: Vec<LinterWarning>,
    }

    let user = current_user(&req)?;

    let new_translation = NewTranslation {
        key: form.key.clone(),
        locale: form.locale.clone(),
        content: form.content.clone(),
        user_id: user.id,
    };

    let connection = database::establish_connection()?;

    diesel::insert_into(translations::table)
        .values(&new_translation)
        .execute(&connection)
        .expect("Error saving new translation");

    let inserted_translation = translations
        .filter(id.eq(sql("last_insert_rowid()")))
        .get_result::<Translation>(&connection)
        .expect("Error getting inserted translation");

    debug!("Translation saved with id={}", inserted_translation.id);

    let mut response = CreateTranslationResponse {
        translation: inserted_translation,
        warnings: Vec::new(),
    };

    let linter = Linter::new(new_translation.locale)
        .unwrap()
        .check(&new_translation.content);

    if linter.is_err() {
        response.warnings = linter.err().unwrap();
    }

    Ok(web::Json(response).with_status(StatusCode::CREATED))
}

#[derive(Deserialize)]
pub struct ShowUrlParams {
    key: String,
}

pub async fn show(params: web::Path<ShowUrlParams>) -> Result<impl Responder, LughError> {
    let connection = database::establish_connection()?;

    let all_translations = translations
        .filter(key.eq(&params.key))
        .load::<Translation>(&connection)
        .expect("Error loading translations");

    debug!("Returns {} translations", all_translations.len());

    Ok(web::Json(all_translations))
}

#[derive(Deserialize)]
pub struct ValidateUrlParams {
    id: i32,
}

pub async fn validate(
    params: web::Path<ValidateUrlParams>,
    req: HttpRequest,
) -> Result<impl Responder, LughError> {
    use crate::schema::translations::dsl::*;
    use diesel::prelude::*;

    let user = current_user(&req)?;

    let connection = database::establish_connection()?;
    let mut translation = find_translation(&connection, params.id)?;

    translation.validator_id = Some(user.id);
    translation.validated_at = Some(now_str());

    let validated = diesel::update(translations.find(translation.id))
        .set(&translation)
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to validate translation with id={}", translation.id));

    Ok(match validated {
        1 => HttpResponse::NoContent(),
        _ => HttpResponse::InternalServerError(),
    })
}

#[derive(Deserialize)]
pub struct DeleteUrlParams {
    key: String,
}

pub async fn delete(params: web::Path<DeleteUrlParams>) -> Result<impl Responder, LughError> {
    let connection = database::establish_connection()?;

    let now = now_str();

    let selected_translations = translations
        .filter(key.eq(&params.key))
        .filter(deleted_at.is_null());

    let deleted = diesel::update(selected_translations)
        .set(deleted_at.eq(now))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to delete translations with key={}", &params.key));

    #[derive(Serialize)]
    struct DeletedResult {
        deleted_translations: usize,
    };

    let payload = DeletedResult {
        deleted_translations: deleted,
    };

    Ok(web::Json(payload).with_status(match deleted {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    }))
}

fn find_translation(
    connection: &SqliteConnection,
    translation_id: i32,
) -> Result<Translation, LughError> {
    use crate::schema::translations::dsl::*;
    use diesel::prelude::*;

    match translations
        .find(translation_id)
        .first::<Translation>(connection)
    {
        Ok(translation) => Ok(translation),
        Err(_) => Err(LughError::NotFound(format!(
            "Can’t find Translation with id={}",
            translation_id
        ))),
    }
}

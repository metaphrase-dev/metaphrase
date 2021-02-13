use actix_web::{web, HttpResponse, Responder};

use crate::{authentication, errors::MetaphraseError};

#[derive(Deserialize)]
pub struct CreateUserFormData {
    email: String,
    password: String,
}

pub async fn create(
    form: web::Json<CreateUserFormData>,
) -> Result<impl Responder, MetaphraseError> {
    let inserted_user = authentication::create_user(&form.email, &form.password)?;

    debug!("User saved with id={}", inserted_user.id);

    Ok(HttpResponse::Created())
}

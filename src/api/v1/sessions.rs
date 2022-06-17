use crate::{
    authentication::{authenticate_user, delete_session},
    errors::MetaphraseError,
};
use actix_web::{http::StatusCode, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::models::{NewSession, Session};

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct LoginFormData {
    email: String,
    password: String,
}

pub async fn login(form: web::Json<LoginFormData>) -> Result<impl Responder, MetaphraseError> {
    let (user, session) = authenticate_user(&form.email, &form.password)?;

    let new_session = NewSession {
        token: session.token,
        user_id: user.id,
        expired_at: session.expired_at,
    };

    Ok(web::Json(new_session)
        .customize()
        .with_status(StatusCode::CREATED))
}

pub async fn logout(request: HttpRequest) -> Result<impl Responder, MetaphraseError> {
    if let Some(current_session) = request.extensions().get::<Session>() {
        delete_session(current_session.token.as_str())?;
    }

    Ok(HttpResponse::NoContent())
}

use actix_web::web;
use actix_web::Responder;

pub mod common;
pub mod configuration;
pub mod sessions;
pub mod translations;
pub mod users;

pub async fn index() -> impl Responder {
    "Welcome to Lugh API!"
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
    cfg.service(
        web::resource("/configuration")
            .name("configuration")
            .route(web::get().to(configuration::index)),
    );
    cfg.service(
        web::resource("/login")
            .name("login")
            .route(web::post().to(sessions::login)),
    );
    cfg.service(
        web::resource("/logout")
            .name("logout")
            .route(web::post().to(sessions::logout)),
    );
    cfg.service(
        web::resource("/translations")
            .name("translations")
            .route(web::get().to(translations::index))
            .route(web::post().to(translations::create)),
    );
    cfg.service(
        web::resource("/translations/{id}/validate")
            .name("translations_validate")
            .route(web::post().to(translations::validate)),
    );
    cfg.service(
        web::resource("/translations/{key}")
            .name("translations_by_key")
            .route(web::get().to(translations::show))
            .route(web::delete().to(translations::delete)),
    );
    cfg.service(
        web::resource("/users")
            .name("users_create")
            .route(web::post().to(users::create)),
    );
}

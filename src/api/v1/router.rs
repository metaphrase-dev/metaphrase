use router::Router;

use errors::LughError;
use super::*;

pub fn router() -> Result<Router, LughError> {
    let mut router = Router::new();

    router.get("/", index, "api");

    router.post("/login", sessions::login, "login");
    router.post("/logout", sessions::logout, "logout");

    router.get("/translations", translations::index, "translations_index");
    router.post("/translations", translations::create, "translations_create");
    router.post("/translations/:id/validate", translations::validate, "translations_validate");
    router.get("/translations/:key", translations::show, "translations_show");
    router.delete("/translations/:key", translations::delete, "translations_delete");

    router.post("/users", users::create, "users_create");

    Ok(router)
}

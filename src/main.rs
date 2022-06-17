use fs::NamedFile;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use std::env;

use actix_files as fs;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, App, HttpServer,
};

#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    check_environment_variables();

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let bind = env::var("METAPHRASE_BIND").unwrap_or_else(|_| "localhost:3000".to_string());
    info!("Start application on {}", bind);

    HttpServer::new(|| {
        App::new()
            .wrap(metaphrase::logger::RequestLogger)
            .service(
                web::scope("/api/v1")
                    .wrap(metaphrase::authentication::middleware::Authentication)
                    .configure(metaphrase::api::v1::config),
            )
            .service(
                fs::Files::new("/", "./src/frontend/dist/")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();

                        async {
                            let response = NamedFile::open("./src/frontend/dist/index.html")?
                                .into_response(&http_req);
                            Ok(ServiceResponse::new(http_req, response))
                        }
                    }),
            )
    })
    .bind(bind.as_str())?
    .run()
    .await
}

fn check_environment_variables() {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

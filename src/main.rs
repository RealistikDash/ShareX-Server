use actix_web::{App, HttpServer, web::Data};
use crate::config::Config;

mod config;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Data::new(Config::load());

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(web::main::main_get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

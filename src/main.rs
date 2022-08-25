use crate::config::Config;
use actix_web::{web::Data, App, HttpServer};
use std::sync::Mutex;

mod config;
mod database;
mod sharex;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load();
    let db = database::sqlite::create_db(config.db_path.clone())
        .expect("Could not load the SQLite database! Cannot continue.");

    let config_arc = Data::new(config);
    let db_mutex = Data::new(Mutex::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(config_arc.clone())
            .app_data(db_mutex.clone())
            .service(web::main::main_get)
            .service(web::sharex::upload_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

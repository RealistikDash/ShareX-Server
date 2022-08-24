use crate::config::Config;
use actix_web::{get, web::Data, Responder};

#[get("/")]
async fn main_get(config: Data<Config>) -> impl Responder {
    format!("Hello! Your private key is {}", config.private_key)
}

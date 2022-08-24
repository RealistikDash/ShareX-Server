use actix_web::{get, Responder, web::Data};
use crate::config::Config;

#[get("/")]
async fn main_get(config: Data<Config>) -> impl Responder {
    format!("Hello! Your private key is {}", config.private_key)
}

use actix_web::{web::Data, post, web::Bytes, Responder, HttpRequest, HttpResponse, http::StatusCode};
use sqlite::Connection;
use std::sync::Mutex;
use crate::sharex::images;
use serde_json::json;

type DatabaseData = Data<Mutex<Connection>>;

/// Fetches the header by `name` from the `req` as a `&str`.
fn header_string(req: &HttpRequest, name: &str) -> Option<String> {
    req.headers().get(name)
    .map(|hv| {
        if let Ok(string) = hv.to_str() {
            Some(string.to_string())
        } else {
            None
        }
    })
    .unwrap_or(None)
}

#[post("/sharex/upload")]
async fn upload_post(req: HttpRequest, db: DatabaseData, image: Bytes) -> impl Responder {
    let token_option = header_string(&req, "Authorisation");
    
    if let Some(token) = token_option {
        HttpResponse::new(StatusCode::ACCEPTED)
        .set_body("Bruh".to_string())
    } else {
        HttpResponse::new(StatusCode::FORBIDDEN)
        .set_body(json!({
            "message": "Auth header error!"
        }).to_string())
    }
}

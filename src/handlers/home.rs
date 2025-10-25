use crate::utils::response::send_success;
use ntex::web;
use serde_json::json;
use std::env;

#[web::get("/")]
pub async fn home() -> impl web::Responder {
    let app_version = env::var("APP_VERSION").unwrap_or_else(|_| "0.0.1".to_string());

    // let body = json!({
    //     "data": { "version": app_version },
    //     "message": "API is running.",
    //     "success": true
    // });

    let data = json!({ "version": app_version });
    // web::HttpResponse::Ok().json(&body)
    send_success("API is running.", data)
}

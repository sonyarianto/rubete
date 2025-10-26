use crate::modules::utils::response::send_success;
use ntex::web;
use serde_json::json;

#[web::get("/healthz")]
pub async fn health_check() -> impl web::Responder {
    let data = json!({ "status": "healthy" });

    send_success("API is healthy", data)
}

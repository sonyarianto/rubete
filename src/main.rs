use dotenvy::dotenv;
use ntex::web;
use serde_json::json;
use std::env;

#[web::get("/")]
async fn home() -> impl web::Responder {
    let app_version = env::var("APP_VERSION").unwrap_or_else(|_| "0.0.1".to_string());

    let body = json!({
        "data": { "version": app_version },
        "message": "API is running.",
        "success": true
    });
    web::HttpResponse::Ok().json(&body)
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey there!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Load port from environment variable or default to 9001
    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "9001".to_string());

    web::HttpServer::new(|| {
        web::App::new()
            .service(home)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", app_port.parse::<u16>().unwrap_or(9001)))?
    .run()
    .await
}

use dotenvy::dotenv;
use ntex::web;
use std::env;
mod handlers;
use handlers::echo::echo;
use handlers::home::home;
use handlers::manual_hello::manual_hello;

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

use dotenvy::dotenv;
use std::env;
mod handlers;
mod server;
use server::run_server;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Load port from environment variable or default to 9001
    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "9001".to_string());
    let app_port = app_port.parse::<u16>().unwrap_or(9001);

    run_server(app_port).await
}

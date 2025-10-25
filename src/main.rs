use dotenvy::dotenv;
use std::env;
mod entity;
mod handlers;
mod server;
mod utils;
use sea_orm::{Database, DbConn};
use server::run_server;

async fn connect_to_mysql_db() -> DbConn {
    let database_url = std::env::var("DB_URL").expect("DB_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    let db = connect_to_mysql_db().await;

    // Load port from environment variable or default to 9001
    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "9001".to_string());
    let app_port = app_port.parse::<u16>().unwrap_or(9001);

    run_server(app_port, db).await
}

use sea_orm::{Database, DbConn};

pub async fn connect_to_mysql_db() -> DbConn {
    let database_url = std::env::var("DB_URL").expect("DB_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

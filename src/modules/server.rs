use crate::handlers::{health_check::health_check, user::create_user};
use crate::modules::handlers::{home::home};
use ntex::web;
use ntex::web::{App, HttpServer};
use sea_orm::DbConn;

pub async fn run_server(app_port: u16, db: DbConn) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // Add DbConn to app state
            .state(db.clone()) // Add DbConn to app state
            // Root routes
            .service(home)
            .service(health_check)
            // Define /v1 scope
            .service(web::scope("/v1").service(create_user).service(home))
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}

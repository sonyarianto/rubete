use crate::handlers::echo::echo;
use crate::handlers::home::home;
use crate::handlers::manual_hello::manual_hello;
use ntex::web::{self, App, HttpServer};
use sea_orm::DbConn;

pub async fn run_server(app_port: u16, db: DbConn) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .state(db.clone()) // Add DbConn to app state
            .service(home)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}

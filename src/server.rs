use crate::handlers::echo::echo;
use crate::handlers::home::home;
use crate::handlers::manual_hello::manual_hello;
use ntex::web;

pub async fn run_server(app_port: u16) -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(home)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}

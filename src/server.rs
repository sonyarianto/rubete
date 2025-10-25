use ntex::web;
use crate::handlers::{home, echo, manual_hello};

pub fn build_app() -> web::App<(), web::Request> {
    web::App::<(), web::Request>::new()
        .service(home::home)
        .service(echo::echo)
        .route("/hey", web::get().to(manual_hello::manual_hello))
}
use ntex::web;

#[web::post("/echo")]
pub async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

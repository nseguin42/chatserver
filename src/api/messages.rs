use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn messages_index() -> impl Responder {
    HttpResponse::Ok().body("Messages index")
}

#[post("/echo")]
pub(crate) async fn echo(body: web::Bytes) -> impl Responder {
    HttpResponse::Ok().body(body)
}

use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn channel_index() -> impl Responder {
    HttpResponse::Ok().body("Channel index")
}

#[get("/{channel}")]
pub(crate) async fn channel(channel: web::Path<String>) -> impl Responder {
    let channel = channel.into_inner();
    HttpResponse::Ok().body(format!("Channel: {}", channel))
}

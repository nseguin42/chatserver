use actix_web::{get, HttpResponse, Responder, web};
use tokio::count;

use crate::server::server_state::ServerState;

#[get("/")]
pub(crate) async fn channel_index() -> impl Responder {
    HttpResponse::Ok().body("Channel index")
}

#[get("/{channel}/messages/{count}")]
pub(crate) async fn channel_get_count(
    data: web::Data<ServerState>,
    path: web::Path<(String, i64)>,
) -> impl Responder {
    let (channel, count) = path.into_inner();

    let repo = &data.repo;
    let messages = repo
        .get_messages_from_channel(&channel, count)
        .await
        .unwrap();

    HttpResponse::Ok().json(messages)
}

#[get("/{channel}/messages")]
pub(crate) async fn channel_get(
    data: web::Data<ServerState>,
    path: web::Path<String>,
) -> impl Responder {
    let count = 10;

    let channel = path.into_inner();
    let repo = &data.repo;
    let messages = repo
        .get_messages_from_channel(&channel, count)
        .await
        .unwrap();

    HttpResponse::Ok().json(messages)
}


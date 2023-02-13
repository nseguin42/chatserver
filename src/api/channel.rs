use actix_web::{get, HttpResponse, Responder, web};

use crate::server::server_state::ServerState;

#[get("/")]
pub(crate) async fn channel_index() -> impl Responder {
    HttpResponse::Ok().body("Channel index")
}

/// extract path info from "/channels/{channel}" URL
/// {channel} - deserializes to a String
#[get("/{channel}")]
pub(crate) async fn channel_get(data: web::Data<ServerState>, path: web::Path<String>) -> impl Responder {
    let channel = path.into_inner();
    let repo = &data.repo;
    let messages = repo.get_messages_from_channel(&channel, 10).await.unwrap();
    let response = serde_json::to_string(&messages).unwrap();

    HttpResponse::Ok().body(response)
}


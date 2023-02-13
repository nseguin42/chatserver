use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;

use crate::models::chat_message::ChatMessage;
use crate::server::server_state::ServerState;

#[get("")]
pub(crate) async fn message_index() -> impl Responder {
    HttpResponse::Ok().body("Messages index")
}

#[post("")]
pub async fn message_post(
    data: web::Data<ServerState>,
    message: web::Json<ChatMessage>,
) -> impl Responder {
    let repo = &data.repo;
    let message = message.into_inner();
    repo.add_message(&message).await.unwrap_or_else(|err| {
        error!("Could not add message: {}", err);
    });

    let response = format!("Successfully added message: {}", message);

    HttpResponse::Ok().body(response)
}

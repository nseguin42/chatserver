use actix_web::{get, HttpResponse, post, Responder, web};
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


#[cfg(test)]
mod tests {
    use actix_web::{test, web};
    use actix_web::test::{init_service, TestRequest};
    use fake::{Fake, Faker};
    use test_context::{AsyncTestContext, test_context};

    use crate::api::message::message_post;
    use crate::api::tests::{ServerTestContext, setup_app};
    use crate::error::Error;
    use crate::models::chat_message::ChatMessage;

    #[test_context(ServerTestContext)]
    #[test]
    async fn api_test_message_post(ctx: &ServerTestContext) -> Result<(), Error> {
        let app = setup_app(&ctx.config)
            .await
            .service(web::scope("/message").service(message_post));

        let service = init_service(app).await;

        let message = Faker.fake::<ChatMessage>();
        let req = TestRequest::post()
            .uri("/message")
            .set_json(&message)
            .to_request();

        let resp = test::call_service(&service, req).await;

        assert!(resp.status().is_success());

        Ok(())
    }
}
use std::sync::Arc;

use actix::{Actor, StreamHandler};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web_actors::ws;
use log::info;

use crate::api::channel;
use crate::config::Config;
use crate::dal::chat_message_repository::ChatMessageRepository;
use crate::error::Error;
use crate::server::server_state::ServerState;

pub mod server_state;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

pub async fn start(config: &Config) -> Result<(), Error> {
    info!("Starting server");

    let mut repo = ChatMessageRepository::new(config).unwrap();
    repo.connect().await?;

    let api_config = &config.api().unwrap();

    let state = ServerState {
        repo: Arc::new(repo),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(web::resource("/ws/").to(index))
            .service(
                web::scope("/channel")
                    .service(channel::channel_index)
                    .service(channel::channel_get),
            )
    })
        .bind(&api_config.address)?
        .run()
        .await?;

    Ok(())
}
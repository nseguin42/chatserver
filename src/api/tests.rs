use std::sync::Arc;

use actix_web::App;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use test_context::AsyncTestContext;

use crate::config::Config;
use crate::dal::chat_message_repository::ChatMessageRepository;
use crate::server::server_state::ServerState;

#[derive(Debug)]
pub(crate) struct ServerTestContext {
    pub(crate) config: Config,
    repo: Arc<ChatMessageRepository>,
}

#[async_trait::async_trait]
impl AsyncTestContext for ServerTestContext {
    async fn setup() -> ServerTestContext {
        let config = Config::load("config.json").await.unwrap();
        let mut repo = ChatMessageRepository::new(&config).unwrap();
        repo.connect().await.unwrap();

        ServerTestContext {
            config,
            repo: Arc::new(repo),
        }
    }

    async fn teardown(self) {}
}

pub(crate) async fn setup_app(
    config: &Config,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=actix_web::Error,
    >,
> {
    let mut repo = ChatMessageRepository::new(config).unwrap();
    let _ = &repo.connect().await;

    App::new().app_data(Data::new(ServerState {
        repo: Arc::new(repo),
    }))
}
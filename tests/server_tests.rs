mod server_tests {
    use std::sync::Arc;

    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
    use actix_web::test::{init_service, TestRequest};
    use actix_web::web::Data;
    use actix_web::{test, web, App};
    use chatserver::api::message::message_post;
    use chatserver::config::Config;
    use chatserver::dal::chat_message_repository::ChatMessageRepository;
    use chatserver::error::Error;
    use chatserver::models::chat_message::ChatMessage;
    use chatserver::server::server_state::ServerState;
    use fake::{Fake, Faker};
    use test_context::{test_context, AsyncTestContext};

    #[derive(Debug)]
    struct ServerTestContext {
        config: Config,
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

    async fn setup_app(
        config: &Config,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            InitError = (),
            Error = actix_web::Error,
        >,
    > {
        let mut repo = ChatMessageRepository::new(config).unwrap();
        let _ = &repo.connect().await;

        App::new().app_data(Data::new(ServerState {
            repo: Arc::new(repo),
        }))
    }

    #[test_context(ServerTestContext)]
    #[test]
    async fn server_test_message_post(ctx: &ServerTestContext) -> Result<(), Error> {
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

mod repository_tests {
    use fake::{Fake, Faker};
    use tokio::test;

    use chatserver::config::Config;
    use chatserver::dal::chat_message_repository::ChatMessageRepository;
    use chatserver::error::Error;
    use chatserver::models::chat_message::ChatMessage;

    async fn setup() -> Result<ChatMessageRepository, Error> {
        let config = Config::load("config.json").await?;
        let mut repo = ChatMessageRepository::new(&config)?;
        repo.connect().await?;

        Ok(repo)
    }

    #[test]
    async fn test_add_message() -> Result<(), Error> {
        let repo = setup().await?;
        let message = Faker.fake::<ChatMessage>();
        repo.add_message(&message).await?;

        Ok(())
    }

    #[test]
    async fn test_get_messages_from_channel() -> Result<(), Error> {
        let repo = setup().await?;

        for _ in 0..10 {
            let message = Faker.fake::<ChatMessage>();
            repo.add_message(&message).await?;
        }

        let messages = repo
            .get_messages_from_channel("test_channel", 10)
            .await
            .unwrap();

        assert_eq!(messages.len(), 10);

        Ok(())
    }

    #[test]
    async fn test_get_messages_from_user() -> Result<(), Error> {
        let repo = setup().await?;
        let message = Faker.fake::<ChatMessage>();
        repo.add_message(&message).await?;
        let messages = repo.get_messages_by_user(&message.username).await?;

        assert_ne!(messages.len(), 0);

        Ok(())
    }
}

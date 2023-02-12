use enum_iterator::Sequence;
use tokio_postgres::types::Type;
use tokio_postgres::{Client, NoTls, Row, Statement, ToStatement};

use crate::config::Config;
use crate::error::Error;
use crate::models::chat_message::ChatMessage;
use crate::utils::connection_string::ConnectionString;

pub(crate) struct RepoStatement {
    statement: String,
    prepared: Option<Statement>,
    types: Vec<Type>,
}

impl RepoStatement {
    fn new(statement: String, types: Vec<Type>) -> Self {
        Self {
            statement,
            types,
            prepared: None,
        }
    }

    pub(crate) async fn prepare(&mut self, client: &Client) -> Result<(), Error> {
        let statement = client.prepare_typed(&self.statement, &self.types);

        // Set a timeout for the statement preparation
        let statement = tokio::time::timeout(std::time::Duration::from_secs(1), statement).await;

        match statement {
            Ok(statement) => {
                self.prepared = Some(statement.unwrap());
                Ok(())
            }
            Err(e) => Err(Error::DbError(format!(
                "Failed to prepare statement {}, error: {}",
                self.statement, e
            )))?,
        }
    }

    fn to_statement(&self) -> &dyn ToStatement {
        let prepared = &self.prepared;
        match prepared {
            Some(x) => x,
            None => &self.statement,
        }
    }

    fn for_chat_repo(s: &ChatRepoStatement) -> Self {
        let statement = s.to_string();
        let types = s.get_types();
        Self::new(statement, types)
    }
}

#[derive(Debug, PartialEq, Sequence)]
enum ChatRepoStatement {
    Insert,
    GetByChannel,
    GetByUser,
}

impl ChatRepoStatement {
    fn to_string(&self) -> String {
        match self {
            ChatRepoStatement::Insert => "INSERT INTO chat_messages (text, channel, username, timestamp) VALUES ($1, $2, $3, $4) RETURNING *".to_string(),
            ChatRepoStatement::GetByChannel => "SELECT * FROM chat_messages WHERE channel = $1 ORDER BY timestamp DESC LIMIT $2".to_string(),
            ChatRepoStatement::GetByUser => "SELECT * FROM chat_messages WHERE username = $1".to_string(),
        }
    }

    fn get_types(&self) -> Vec<Type> {
        match self {
            ChatRepoStatement::Insert => {
                vec![Type::TEXT, Type::TEXT, Type::TEXT, Type::TIMESTAMPTZ]
            }
            ChatRepoStatement::GetByChannel => vec![Type::TEXT],
            ChatRepoStatement::GetByUser => vec![Type::TEXT],
        }
    }
}

pub struct ChatMessageRepository {
    connection_string: ConnectionString,
    pub client: Option<Client>,
    statements: Vec<RepoStatement>,
}

impl ChatMessageRepository {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let connection_string = config.db()?;

        Ok(Self {
            connection_string,
            client: None,
            statements: Vec::new(),
        })
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let (client, connection) =
            tokio_postgres::connect(&self.connection_string.to_string(), NoTls).await?;

        self.client = Some(client);
        self.prepare_statements().await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(())
    }

    /// This setup prevents us from accidentally using  the wrong statement,
    /// without having to use a hashmap every time we want to use a statement.
    async fn prepare_statements(&mut self) -> Result<(), Error> {
        let _client = self.client.as_ref().unwrap();

        let statements: Vec<RepoStatement> = vec![
            ChatRepoStatement::Insert,
            ChatRepoStatement::GetByChannel,
            ChatRepoStatement::GetByUser,
        ]
        .iter()
        .map(|s| RepoStatement::for_chat_repo(s))
        .collect();

        // This is currently broken, causes indefinite hang.
        /*
        for statement in statements.iter_mut() {
            debug!("Preparing statement: {}", statement.statement);
            statement.prepare(client).await?;
        }
         */

        self.statements = statements;

        Ok(())
    }

    pub async fn get_messages_from_channel(
        &self,
        channel: &str,
        num_to_get: i64,
    ) -> Result<Vec<ChatMessage>, tokio_postgres::Error> {
        let client = self.client.as_ref().unwrap();

        let rows = client
            .query(
                &ChatRepoStatement::GetByChannel.to_string(),
                &[&channel, &num_to_get],
            )
            .await?;

        let messages = from_rows(rows);

        Ok(messages)
    }

    pub async fn add_message(&self, message: &ChatMessage) -> Result<(), Error> {
        let client = self.client.as_ref().unwrap();

        client
            .execute(
                &ChatRepoStatement::Insert.to_string(),
                &[
                    &message.text,
                    &message.channel,
                    &message.username,
                    &message.timestamp,
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn get_messages_by_user(&self, username: &str) -> Result<Vec<ChatMessage>, Error> {
        let client = self.client.as_ref().unwrap();

        let rows = client
            .query(&ChatRepoStatement::GetByUser.to_string(), &[&username])
            .await?;

        let messages = from_rows(rows);

        Ok(messages)
    }
}

fn from_rows(rows: Vec<Row>) -> Vec<ChatMessage> {
    let mut messages = Vec::new();

    for row in rows {
        messages.push(ChatMessage::from(row));
    }

    messages
}

use std::fmt::Display;

use actix_web::cookie::time;
use fake::{Dummy, Fake, Faker};
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub text: String,
    pub username: String,
    pub channel: String,
    pub timestamp: OffsetDateTime,
}

impl ChatMessage {
    pub fn new(text: String, username: String, channel: String, timestamp: OffsetDateTime) -> Self {
        ChatMessage {
            text,
            username,
            channel,
            timestamp,
        }
    }
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.timestamp, self.username, self.text)
    }
}

impl From<Row> for ChatMessage {
    fn from(row: Row) -> Self {
        Self {
            text: row.get("text"),
            channel: row.get("channel"),
            username: row.get("username"),
            timestamp: row.get("timestamp"),
        }
    }
}

impl Dummy<Faker> for ChatMessage {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let fake_timestamp = Faker.fake_with_rng::<u32, R>(rng).into();

        Self {
            text: Faker.fake_with_rng(rng),
            username: Faker.fake_with_rng(rng),
            channel: Faker.fake_with_rng(rng),
            timestamp: OffsetDateTime::from_unix_timestamp(fake_timestamp).unwrap(),
        }
    }
}

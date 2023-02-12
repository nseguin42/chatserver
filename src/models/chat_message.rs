use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, ToSql};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Dummy, FromSql, ToSql)]
pub struct ChatMessage {
    pub text: String,
    pub username: String,
    pub channel: String,
    #[serde(deserialize_with = "from_ts")]
    #[serde(serialize_with = "to_ts")]
    pub timestamp: DateTime<Utc>,
}

impl ChatMessage {
    pub fn new(text: String, username: String, channel: String, timestamp: DateTime<Utc>) -> Self {
        ChatMessage {
            text,
            username,
            channel,
            timestamp,
        }
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

// PostgreSQL timestamp precision differs from chrono's
// so we can't #[derive(PartialEq)] for ChatMessage
impl PartialEq for ChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.username == other.username
            && self.channel == other.channel
            && self.timestamp.timestamp_millis() == other.timestamp.timestamp_millis()
    }
}

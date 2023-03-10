use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;
use futures::StreamExt;
use log::debug;
use time::OffsetDateTime;

use crate::models::chat_message::ChatMessage;

pub(crate) struct ChatStream<'a> {
    pub(crate) created_at: OffsetDateTime,
    pub(crate) updated_at: OffsetDateTime,
    stream: Pin<Box<dyn Stream<Item = ChatMessage> + 'a>>,
}

impl<'a> ChatStream<'a> {
    pub(crate) fn new<T>(stream: T) -> Self
    where
        T: Stream<Item = ChatMessage> + Send + Sync + 'a,
    {
        let stream = Box::pin(stream);
        let now = OffsetDateTime::now_local().unwrap();

        Self {
            stream,
            created_at: now,
            updated_at: now,
        }
    }

    pub(crate) fn with_adapter<T, F>(stream: T, adapter: F) -> Self
    where
        T: Stream<Item = ChatMessage> + Send + Sync + 'a,
        F: FnMut(ChatMessage) -> ChatMessage + Send + Sync + 'a,
    {
        let stream = Box::pin(stream.map(adapter));
        let now = OffsetDateTime::now_local().unwrap();
        Self {
            stream,
            created_at: now,
            updated_at: now,
        }
    }

    pub(crate) async fn start(&self) {
        debug!("Starting chat stream");
    }

    pub(crate) async fn stop(&self) {
        debug!("Stopping chat stream");
    }

    pub(crate) async fn await_next(&mut self) -> Option<ChatMessage> {
        self.stream.next().await
    }
}

impl<'a> Stream for ChatStream<'a> {
    type Item = ChatMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let val = self.stream.as_mut().poll_next(cx);
        self.updated_at = OffsetDateTime::now_local().unwrap();
        val
    }
}

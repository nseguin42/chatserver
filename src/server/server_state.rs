use std::sync::Arc;

use crate::dal::chat_message_repository::ChatMessageRepository;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub repo: Arc<ChatMessageRepository>,
}

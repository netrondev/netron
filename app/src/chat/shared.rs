use serde::{Deserialize, Serialize};

use crate::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user_id: RecordId,
    pub username: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    UserJoined { username: String },
    UserLeft { username: String },
    Message(ChatMessage),
}

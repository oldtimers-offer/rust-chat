use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WebSocketMessageType {
    NewMessage,
    UsersList,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    pub message: Option<ChatMessages>,
    pub users: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatMessages {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime,
}

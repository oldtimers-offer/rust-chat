use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WebSocketMessageType {
    NewMessage,
    UsersList,
    UsernameChange,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    pub message: Option<ChatMessages>,
    pub users: Option<Vec<String>>,
    pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatMessages {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime,
}

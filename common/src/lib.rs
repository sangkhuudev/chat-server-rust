use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WebsocketMessageType {
    NewMessage,
    UsersList,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebsocketMessage {
    pub message_type: WebsocketMessageType,
    pub message: Option<ChatMessage>,
    pub users: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime
}
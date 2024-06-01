use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime
}
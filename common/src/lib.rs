use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessages {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime,
}

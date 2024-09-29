use crate::domain::entities::id::Id;
use crate::domain::entities::Conversation;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageFrom {
    User(String),
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Message {
    pub id: Id<Message>,
    pub conversation_id: Id<Conversation>,
    pub from: MessageFrom,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

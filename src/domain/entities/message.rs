use crate::domain::entities::conversation::ConversationId;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageFrom {
    User(uuid::Uuid),
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub conversation_id: ConversationId,
    pub from: MessageFrom,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

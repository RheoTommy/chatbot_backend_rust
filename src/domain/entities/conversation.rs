use crate::domain::entities::user::UserId;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationId(uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: ConversationId,
    pub user_id: UserId,
    pub title: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

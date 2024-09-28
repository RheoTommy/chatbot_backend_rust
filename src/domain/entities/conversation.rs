use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

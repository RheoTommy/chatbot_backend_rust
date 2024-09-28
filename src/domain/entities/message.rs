use std::time::SystemTime;

pub enum MessageFrom {
    User(uuid::Uuid),
    Bot,
}

pub struct Message {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub from: MessageFrom,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

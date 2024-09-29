use crate::domain::entities::id::Id;
use crate::domain::entities::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Conversation {
    pub id: Id<Conversation>,
    pub user_id: Id<User>,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

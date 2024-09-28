use crate::domain::entities::Conversation;
use anyhow::Error;

pub trait ConversationRepository {
    fn get(&self, id: uuid::Uuid) -> Result<Conversation, Error>;
    fn put(&mut self, conversation: Conversation) -> Result<(), Error>;
    fn delete(&mut self, id: uuid::Uuid) -> Result<(), Error>;
    fn query_by_user_id(&self, user_id: uuid::Uuid) -> Result<Vec<Conversation>, Error>;
}

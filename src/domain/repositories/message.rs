use crate::domain::entities::Message;
use anyhow::Error;

pub trait MessageRepository {
    fn get(&self, id: uuid::Uuid) -> Result<Message, Error>;
    fn put(&self, message: Message) -> Result<(), Error>;
    fn delete(&self, id: uuid::Uuid) -> Result<(), Error>;
    fn query_by_conversation_id(&self, conversation_id: uuid::Uuid) -> Result<Vec<Message>, Error>;
}

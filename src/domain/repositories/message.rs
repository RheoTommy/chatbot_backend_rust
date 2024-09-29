use crate::domain::entities::{Conversation, Id, Message};
use crate::domain::repositories::errors::RepositoryError;

pub trait MessageRepository {
    async fn get(&self, id: Id<Message>) -> Result<Message, RepositoryError>;

    async fn put(&self, message: Message) -> Result<Message, RepositoryError>;

    async fn delete(&self, id: Id<Message>) -> Result<Message, RepositoryError>;

    async fn query_by_conversation_id(
        &self,
        conversation_id: Id<Conversation>,
    ) -> Result<Vec<Message>, RepositoryError>;
}

use crate::domain::entities::{ConversationId, Message, MessageId};
use crate::domain::repositories::errors::RepositoryError;

pub trait MessageRepository {
    async fn get(&self, id: MessageId) -> Result<Message, RepositoryError>;

    async fn put(&self, message: Message) -> Result<Message, RepositoryError>;

    async fn delete(&self, id: MessageId) -> Result<Message, RepositoryError>;

    async fn query_by_conversation_id(
        &self,
        conversation_id: ConversationId,
    ) -> Result<Vec<Message>, RepositoryError>;
}

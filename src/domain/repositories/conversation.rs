use crate::domain::entities::{Conversation, ConversationId};
use crate::domain::repositories::errors::RepositoryError;

pub trait ConversationRepository {
    async fn get(&self, id: ConversationId) -> Result<Conversation, RepositoryError>;

    async fn put(&self, conversation: Conversation) -> Result<Conversation, RepositoryError>;

    async fn delete(&self, id: ConversationId) -> Result<Conversation, RepositoryError>;

    async fn query_by_user_id(
        &self,
        user_id: ConversationId,
    ) -> Result<Vec<Conversation>, RepositoryError>;
}

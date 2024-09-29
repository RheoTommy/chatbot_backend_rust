use crate::domain::entities::{Conversation, Id, User};
use crate::domain::repositories::errors::RepositoryError;

pub trait ConversationRepository {
    async fn get(&self, id: Id<Conversation>) -> Result<Conversation, RepositoryError>;

    async fn put(&self, conversation: Conversation) -> Result<Conversation, RepositoryError>;

    async fn delete(&self, id: Id<Conversation>) -> Result<(), RepositoryError>;

    async fn query_by_user_id(
        &self,
        user_id: Id<User>,
    ) -> Result<Vec<Conversation>, RepositoryError>;
}

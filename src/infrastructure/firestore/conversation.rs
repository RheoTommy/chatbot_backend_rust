use crate::domain;
use crate::domain::entities::{Conversation, Id, User};
use crate::domain::repositories::errors::RepositoryError;
use derive_new::new;
use firestore::{path, paths, FirestoreDb, FirestoreQueryDirection};
use futures_util::TryStreamExt;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct FirestoreConversationRepository {
    client: Arc<FirestoreDb>,
    collection_name: Arc<str>,
}

impl domain::repositories::ConversationRepository for FirestoreConversationRepository {
    async fn get(&self, id: Id<Conversation>) -> Result<Conversation, RepositoryError> {
        self.client
            .fluent()
            .select()
            .by_id_in(self.collection_name.as_ref())
            .obj()
            .one(id.to_string())
            .await
            .map_err(|e| RepositoryError::Unexpected(Box::new(e)))?
            .ok_or(RepositoryError::NotFound(id.to_string()))
    }

    async fn put(&self, conversation: Conversation) -> Result<Conversation, RepositoryError> {
        self.client
            .fluent()
            .insert()
            .into(self.collection_name.as_ref())
            .document_id(conversation.id.to_string())
            .object(&conversation)
            .execute::<Conversation>()
            .await
            .map_err(|e| RepositoryError::Unexpected(Box::new(e)))
    }

    async fn delete(&self, id: Id<Conversation>) -> Result<(), RepositoryError> {
        self.client
            .fluent()
            .delete()
            .from(self.collection_name.as_ref())
            .document_id(id.to_string())
            .execute()
            .await
            .map_err(|e| RepositoryError::Unexpected(Box::new(e)))
    }

    async fn query_by_user_id(
        &self,
        user_id: Id<User>,
    ) -> Result<Vec<Conversation>, RepositoryError> {
        let stream = self
            .client
            .fluent()
            .select()
            .fields(paths!(Conversation::{id, title, user_id, created_at, updated_at}))
            .from(self.collection_name.as_ref())
            .filter(|q| q.field(path!(Conversation::user_id)).eq(user_id.clone()))
            .order_by([(
                path!(Conversation::created_at),
                FirestoreQueryDirection::Descending,
            )])
            .obj()
            .stream_query_with_errors()
            .await
            .map_err(|e| RepositoryError::Unexpected(Box::new(e)))?;

        stream
            .try_collect()
            .await
            .map_err(|e| RepositoryError::Unexpected(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::conversation::ConversationRepository;
    use crate::infrastructure::firestore::client::from_env;
    use chrono::DateTime;
    use itertools::Itertools;
    use std::sync::Arc;
    use std::time::SystemTime;
    use uuid::Uuid;

    const COLLECTION_NAME: &str = "test-conversations";

    fn gen_uuid_sample(suffix: u64) -> Uuid {
        Uuid::parse_str(&format!("00000000-0000-4000-a000-{:012}", suffix)).unwrap()
    }

    #[tokio::test]
    async fn test_conversation_repository() -> anyhow::Result<()> {
        let base_time = DateTime::from(SystemTime::UNIX_EPOCH);

        let client = Arc::new(from_env().await?);

        let repo = FirestoreConversationRepository::new(client, Arc::from(COLLECTION_NAME));

        let n = 5;
        let conversations = (0..n)
            .map(|i| Conversation {
                id: Id::new(gen_uuid_sample(i)),
                user_id: Id::new(gen_uuid_sample(i % 2)),
                title: format!("title-{}", i),
                created_at: base_time,
                updated_at: base_time,
            })
            .collect_vec();

        // Get from empty repository
        let id = conversations[0].id.clone();
        let result = repo.get(id.clone()).await;
        assert!(result.is_err(), "result: {:?}", result);
        let err = result.err().unwrap();
        assert!(
            matches!(
                err,
                RepositoryError::NotFound(ref id) if id == &conversations[0].id.to_string()
            ),
            "err: {:?}",
            err
        );

        // Query by user_id from empty repository
        let user_id = conversations[0].user_id.clone();
        let result = repo.query_by_user_id(user_id.clone()).await;
        assert!(result.is_ok(), "result: {:?}", &result);
        assert_eq!(result?, vec![]);

        // Put
        for conversation in conversations.iter() {
            let result = repo.put(conversation.clone()).await;
            assert!(result.is_ok(), "result: {:?}", &result);
            assert_eq!(result?, conversation.clone());
        }

        // Get
        for conversation in conversations.iter() {
            let id = conversation.id.clone();
            let result = repo.get(id.clone()).await;
            assert!(result.is_ok(), "result: {:?}", &result);
            assert_eq!(result?, conversation.clone());
        }

        // Query by user_id
        for user_id in conversations.iter().map(|c| c.user_id.clone()).unique() {
            let expected = conversations
                .iter()
                .filter(|c| c.user_id == user_id)
                .sorted_by_key(|c| c.created_at)
                .rev()
                .cloned()
                .collect_vec();
            let result = repo.query_by_user_id(user_id.clone()).await;
            assert!(result.is_ok(), "result: {:?}", &result);
            assert_eq!(result?, expected);
        }

        // Delete
        for conversation in conversations.iter() {
            let id = conversation.id.clone();
            let result = repo.delete(id.clone()).await;
            assert!(result.is_ok(), "result: {:?}", &result);
        }

        Ok(())
    }
}

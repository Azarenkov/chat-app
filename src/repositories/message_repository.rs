use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::Collection;

use crate::{models::message::Message, services::message_service::MessageRepositoryAbstract};

use super::errors::RepositoryError;

pub struct MessageRepository {
    collection: Collection<Message>,
}

impl MessageRepository {
    pub fn new(collection: Collection<Message>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl MessageRepositoryAbstract for MessageRepository {
    async fn save(&self, message: &Message) -> Result<(), RepositoryError> {
        self.collection.insert_one(message).await?;
        Ok(())
    }
    async fn get_by_recipient(&self, recipient: &str) -> Result<Vec<Message>, RepositoryError> {
        let cursor = self
            .collection
            .find(mongodb::bson::doc! {"recipient": recipient})
            .await?;
        let messages: Vec<Message> = cursor.try_collect().await?;
        Ok(messages)
    }
}

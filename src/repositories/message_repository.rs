use std::sync::Arc;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use crate::{models::message::Message, services::message_service::MessageRepositoryAbstract};

use super::errors::RepositoryError;

pub struct MessageRepository {
    collection: Collection<Message>,
}

impl MessageRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            collection: database.collection("messages"),
        }
    }
}

#[async_trait]
impl MessageRepositoryAbstract for MessageRepository {
    async fn save(&self, message: &Message) -> Result<(), RepositoryError> {
        self.collection.insert_one(message).await?;
        Ok(())
    }
    async fn get_by_recipient(&self, user_login: &str) -> Result<Vec<Message>, RepositoryError> {
        let filter = doc! {
            "$or": [
                { "sender": user_login },
                { "recipient": user_login }
            ]
        };
        let cursor = self.collection.find(filter).await?;
        let messages: Vec<Message> = cursor.try_collect().await?;
        Ok(messages)
    }
}

use async_trait::async_trait;

use crate::{models::message::Message, repositories::errors::RepositoryError};

#[async_trait]
pub trait MessageRepositoryAbstract: Send + Sync {
    async fn save(&self, message: &Message) -> Result<(), RepositoryError>;
    async fn get_by_recipient(&self, recipient: &str) -> Result<Vec<Message>, RepositoryError>;
}

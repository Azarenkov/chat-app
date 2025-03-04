use async_trait::async_trait;

use crate::models::message::Message;

use super::errors::ServiceError;

#[async_trait]
pub trait MessageServiceAbstract: Send + Sync {
    async fn send_message(&self, message: &mut Message) -> Result<(), ServiceError>;
    async fn validate_token(&self, token: &str) -> Result<String, ServiceError>;
    async fn get_messages(&self, token: &str) -> Result<Vec<Message>, ServiceError>;
}

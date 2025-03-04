use actix_ws::Session;
use async_trait::async_trait;

use super::errors::ServiceError;

#[async_trait]
pub trait WebSocketSenderAbstract: Send + Sync {
    async fn send(&self, recipient: &str, message: &str) -> Result<(), ServiceError>;
    async fn register(&self, user_login: String, session: Session);
    async fn unregister(&self, user_login: &str);
}

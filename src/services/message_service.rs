use std::sync::Arc;

use async_trait::async_trait;

use crate::{models::message::Message, repositories::errors::RepositoryError};

use super::{
    auth_service::UserRepositoryAbstract, errors::ServiceError,
    jwt_service_abstract::JwtServiceAbstract, message_service_abstract::MessageServiceAbstract,
    web_socket_sender_abstract::WebSocketSenderAbstract,
};

#[async_trait]
pub trait MessageRepositoryAbstract: Send + Sync {
    async fn save(&self, message: &Message) -> Result<(), RepositoryError>;
    async fn get_by_recipient(&self, user_login: &str) -> Result<Vec<Message>, RepositoryError>;
}

pub struct MessageService {
    user_repository: Arc<dyn UserRepositoryAbstract>,
    message_repository: Box<dyn MessageRepositoryAbstract>,
    jwt_service: Arc<dyn JwtServiceAbstract>,
    websocket_sender: Arc<dyn WebSocketSenderAbstract>,
}

impl MessageService {
    pub fn new(
        user_repository: Arc<dyn UserRepositoryAbstract>,
        message_repository: Box<dyn MessageRepositoryAbstract>,
        jwt_service: Arc<dyn JwtServiceAbstract>,
        websocket_sender: Arc<dyn WebSocketSenderAbstract>,
    ) -> Self {
        Self {
            user_repository,
            message_repository,
            jwt_service,
            websocket_sender,
        }
    }
}

#[async_trait]
impl MessageServiceAbstract for MessageService {
    async fn send_message(&self, message: &Message) -> Result<(), ServiceError> {
        if self.user_repository.find(&message.recipient).await.is_ok() {
            return Err(ServiceError::InvalidRecipient(
                message.recipient.to_string(),
            ));
        };
        self.message_repository.save(message).await?;

        let msg_json = serde_json::to_string(message).map_err(|_| ServiceError::InternalError)?;
        self.websocket_sender
            .send(&message.recipient, &msg_json)
            .await?;

        Ok(())
    }
    async fn validate_token(&self, token: &str) -> Result<String, ServiceError> {
        match self.jwt_service.validate_token(token) {
            Ok(login) => Ok(login),
            Err(_) => Err(ServiceError::InvalidToken(token.to_string())),
        }
    }
    async fn get_messages(&self, token: &str) -> Result<Vec<Message>, ServiceError> {
        let login = self.validate_token(token).await?;

        let messages = self.message_repository.get_by_recipient(&login).await?;
        Ok(messages)
    }
}

use std::{collections::HashMap, sync::Arc};

use actix_ws::Session;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::services::{errors::ServiceError, web_socket_sender_abstract::WebSocketSenderAbstract};

pub struct WebSocketSender {
    pub clients: Arc<RwLock<HashMap<String, Session>>>,
}

impl WebSocketSender {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl WebSocketSenderAbstract for WebSocketSender {
    async fn send(&self, recipient: &str, message: &str) -> Result<(), ServiceError> {
        let clients = self.clients.read().await;
        if let Some(session) = clients.get(recipient) {
            session
                .clone()
                .text(message)
                .await
                .map_err(|_| ServiceError::InternalError)?;
        }
        Ok(())
    }
    async fn register(&self, user_login: String, session: Session) {
        let mut clients = self.clients.write().await;
        clients.insert(user_login, session);
    }

    async fn unregister(&self, user_login: &str) {
        let mut clients = self.clients.write().await;
        clients.remove(user_login);
    }
}

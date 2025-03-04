use std::sync::Arc;

use actix_web::web;

use crate::services::{
    auth_service_abstract::AuthServiceAbstract, message_service_abstract::MessageServiceAbstract,
    web_socket_sender_abstract::WebSocketSenderAbstract,
};

pub struct AppState {
    pub auth_service: Box<dyn AuthServiceAbstract>,
    pub message_service: Box<dyn MessageServiceAbstract>,
    pub web_socket_sender: Arc<dyn WebSocketSenderAbstract>,
}

impl AppState {
    pub fn new(
        auth_service: Box<dyn AuthServiceAbstract>,
        message_service: Box<dyn MessageServiceAbstract>,
        web_socket_sender: Arc<dyn WebSocketSenderAbstract>,
    ) -> web::Data<Self> {
        web::Data::new(Self {
            auth_service,
            message_service,
            web_socket_sender,
        })
    }
}

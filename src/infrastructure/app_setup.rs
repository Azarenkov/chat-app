use std::{error::Error, sync::Arc};

use actix_web::web::Data;

use crate::{
    config::Config,
    controllers::shared::app_state::AppState,
    repositories::{message_repository::MessageRepository, user_repository::UserRepository},
    services::{
        auth_service::AuthService, auth_service_abstract::AuthServiceAbstract,
        message_service::MessageService, message_service_abstract::MessageServiceAbstract,
        web_socket_sender_abstract::WebSocketSenderAbstract,
    },
};

use super::{
    db::connection::connect, jwt::jwt_service::JwtService, ws::web_socket_sender::WebSocketSender,
};

pub struct AppDependencies {
    pub auth_service: Box<dyn AuthServiceAbstract>,
    pub message_service: Box<dyn MessageServiceAbstract>,
    pub web_socket_sender: Arc<dyn WebSocketSenderAbstract>,
}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies, Box<dyn Error>> {
    let client_db = connect(&config.mongo_uri).await?;
    let client_db = Arc::new(client_db);
    let web_socket_sender = Arc::new(WebSocketSender::new());

    let user_repository = Arc::new(UserRepository::new(client_db.clone()));
    let message_repository = Box::new(MessageRepository::new(client_db));

    let jwt_service = Arc::new(JwtService::new(config.jwt_secret.to_string()));
    let auth_service = Box::new(AuthService::new(
        user_repository.clone(),
        jwt_service.clone(),
    ));
    let message_service = Box::new(MessageService::new(
        user_repository,
        message_repository,
        jwt_service,
        web_socket_sender.clone(),
    ));

    let app_dependencies = AppDependencies {
        auth_service,
        message_service,
        web_socket_sender,
    };
    Ok(app_dependencies)
}

pub fn create_app_state(app_dependencies: AppDependencies) -> Data<AppState> {
    AppState::new(
        app_dependencies.auth_service,
        app_dependencies.message_service,
        app_dependencies.web_socket_sender,
    )
}

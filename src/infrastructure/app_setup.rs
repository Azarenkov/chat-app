use std::error::Error;

use actix_web::web::Data;

use crate::{
    config::Config,
    controllers::shared::app_state::AppState,
    repositories::user_repository::UserRepository,
    services::{auth_service::AuthService, auth_service_abstract::AuthServiceAbstract},
};

use super::{db::connection::connect, jwt::jwt_service::JwtService};

pub struct AppDependencies {
    pub auth_service: Box<dyn AuthServiceAbstract>,
}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies, Box<dyn Error>> {
    let client_db = connect(&config.mongo_uri).await?;
    let user_repository = Box::new(UserRepository::new(client_db));
    let jwt_service = Box::new(JwtService::new(config.jwt_secret.to_string()));
    let auth_service = Box::new(AuthService::new(user_repository, jwt_service));

    let app_dependencies = AppDependencies { auth_service };
    Ok(app_dependencies)
}

pub fn create_app_state(auth_service: Box<dyn AuthServiceAbstract>) -> Data<AppState> {
    AppState::new(auth_service)
}

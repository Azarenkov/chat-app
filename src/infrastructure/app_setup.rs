use std::error::Error;

use crate::{
    config::Config,
    repositories::user_repository::UserRepository,
    services::{auth_service::AuthService, auth_service_abstract::AuthServiceAbstract},
};

use super::db::connection::connect;

pub struct AppDependencies {
    pub auth_service: Box<dyn AuthServiceAbstract>,
}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies, Box<dyn Error>> {
    let client_db = connect(&config.mongo_uri).await?;
    let user_repository = Box::new(UserRepository::new(client_db.collection("users")));
    let auth_service = Box::new(AuthService::new(user_repository));

    let app_dependencies = AppDependencies { auth_service };
    Ok(app_dependencies)
}

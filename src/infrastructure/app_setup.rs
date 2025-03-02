use std::error::Error;

use crate::config::Config;

use super::db::connection::connect;

pub struct AppDependencies {}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies, Box<dyn Error>> {
    let client_db = connect(&config.mongo_uri).await?;
    Ok(AppDependencies {})
}

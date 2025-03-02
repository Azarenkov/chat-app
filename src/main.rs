use std::error::Error;

use config::Config;
use infrastructure::app_setup::initialize_dependencies;

mod config;
mod infrastructure;
mod models;
mod repositories;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    let deps = initialize_dependencies(&config).await?;

    println!("Hello, world!");
    Ok(())
}

use std::error::Error;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use config::Config;
use controllers::{auth_controller::auth_routes, message_controller::message_routes};
use infrastructure::app_setup::{create_app_state, initialize_dependencies};

mod config;
mod controllers;
mod infrastructure;
mod models;
mod repositories;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let config = Config::from_env()?;
    let deps = initialize_dependencies(&config).await?;
    let app_state = create_app_state(deps);

    let _guard = sentry::init((
        config.sentry.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let address = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(sentry_actix::Sentry::new())
            .app_data(app_state.clone())
            .configure(auth_routes)
            .configure(message_routes)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

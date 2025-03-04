use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use sentry::{capture_message, Level};

use crate::{
    controllers::shared::app_state::AppState,
    models::{api_errors::ApiError, jwt::JwtToken, user::User},
};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(registration).service(login));
}

#[post("/registration")]
async fn registration(
    user: Json<User>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    app_state.auth_service.register(&user).await?;
    capture_message(&format!("User registered: {}", user.login), Level::Info);
    Ok(HttpResponse::Ok().json("User was created"))
}

#[post("/login")]
async fn login(user: Json<User>, app_state: Data<AppState>) -> Result<HttpResponse, ApiError> {
    let jwt_token = app_state.auth_service.login(&user).await?;
    let response = JwtToken { token: jwt_token };
    capture_message(&format!("User logged in: {}", user.login), Level::Info);
    Ok(HttpResponse::Ok().json(response))
}

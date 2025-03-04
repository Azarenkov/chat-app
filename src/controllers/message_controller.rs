use actix_web::{
    get,
    web::{self, Data, Payload},
    HttpRequest, HttpResponse,
};
use actix_ws::{handle, AggregatedMessage};
use futures::StreamExt;
use sentry::{capture_message, Level};

use crate::{
    controllers::shared::app_state::AppState,
    models::{api_errors::ApiError, message::Message},
    services::errors::ServiceError,
};

pub fn message_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messages")
            .service(message)
            .service(get_old_messages),
    );
}

#[get("/old_messages")]
async fn get_old_messages(
    req: HttpRequest,
    app_state: Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ApiError::BadRequest {
            field: "Missing authorization header".to_string(),
        })?
        .to_str()
        .map_err(|_| ApiError::Unauthorized {
            field: "Invalid Authorization header format".to_string(),
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::Unauthorized {
            field: auth_header.to_string(),
        })?;
    let response = app_state.message_service.get_messages(token).await?;
    capture_message(
        &format!("User got all messages with token: {}", token),
        Level::Info,
    );
    Ok(HttpResponse::Ok().json(response))
}

#[get("/ws")]
async fn message(
    req: HttpRequest,
    stream: Payload,
    app_state: Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let query = req.query_string();
    let token = query
        .split('&')
        .find(|param| param.starts_with("token="))
        .and_then(|param| param.strip_prefix("token="))
        .ok_or_else(|| ApiError::BadRequest {
            field: "Missing token".to_string(),
        })?;

    let user_login = app_state.message_service.validate_token(token).await?;

    let (res, mut session, stream) =
        handle(&req, stream).map_err(|_| ApiError::InternalServerError)?;
    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    app_state
        .web_socket_sender
        .register(user_login.clone(), session.clone())
        .await;

    capture_message(
        &format!("WebSocket connected for user: {}", user_login),
        Level::Info,
    );

    actix_web::rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    if let Ok(message) = serde_json::from_str::<Message>(&text) {
                        match app_state.message_service.send_message(&message).await {
                            Ok(()) => {
                                capture_message(
                                    &format!(
                                        "Message sent from {} to {}",
                                        message.sender, message.recipient
                                    ),
                                    Level::Info,
                                );
                            }
                            Err(ServiceError::InvalidRecipient(msg)) => {
                                let error_response = serde_json::json!({
                                    "error": format!("Recipient not found: {}", msg),
                                    "recipient": message.recipient
                                });
                                let error_json = serde_json::to_string(&error_response).unwrap();
                                session.text(error_json).await.unwrap_or(());
                                capture_message(
                                    &format!("Invalid recipient: {}", message.recipient),
                                    Level::Warning,
                                );
                            }
                            Err(e) => {
                                capture_message(
                                    &format!("Error sending message: {:?}", e),
                                    Level::Error,
                                );
                            }
                        };
                    } else {
                        capture_message("Failed to deserialize WebSocket message", Level::Warning);
                    }
                }
                Ok(AggregatedMessage::Close(_)) => {
                    app_state.web_socket_sender.unregister(&user_login).await;
                    capture_message(
                        &format!("WebSocket disconnected for user: {}", user_login),
                        Level::Info,
                    );
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}

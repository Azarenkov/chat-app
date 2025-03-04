use actix_web::{
    get, post,
    web::{self, Data, Json, Payload},
    HttpRequest, HttpResponse,
};
use actix_ws::{handle, AggregatedMessage};
use futures::StreamExt;

use crate::{
    controllers::shared::app_state::AppState,
    models::{api_errors::ApiError, jwt::JwtToken, message::Message},
    services::errors::ServiceError,
};

pub fn message_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messages")
            .service(message)
            .service(get_old_messages),
    );
}

#[post("/old_messages")]
async fn get_old_messages(
    token: Json<JwtToken>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let response = app_state.message_service.get_messages(&token.token).await?;
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

    actix_web::rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    if let Ok(message) = serde_json::from_str::<Message>(&text) {
                        match app_state.message_service.send_message(&message).await {
                            Ok(()) => {}
                            Err(ServiceError::InvalidRecipient) => {
                                let error_response = serde_json::json!({
                                    "error": "Recipient not found",
                                    "recipient": message.recipient
                                });
                                let error_json = serde_json::to_string(&error_response).unwrap();
                                session.text(error_json).await.unwrap_or(());
                            }
                            Err(e) => {
                                eprintln!("Error sending message: {:?}", e);
                            }
                        };
                    }
                }
                Ok(AggregatedMessage::Close(_)) => {
                    app_state.web_socket_sender.unregister(&user_login).await;
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}

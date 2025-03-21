use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use sentry::{capture_message, Level};
use serde::Serialize;

use crate::services::errors::ServiceError;

#[derive(Debug, Serialize, Display)]
pub enum ApiError {
    #[display("User already exist")]
    RegistrationError { field: String },

    #[display("Invalid credentials")]
    Unauthorized { field: String },

    #[display("An internal error occurred. Please try again later.")]
    InternalServerError,

    #[display("Bad request")]
    BadRequest { field: String },
}

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::DatabaseError(_e) => ApiError::InternalServerError,
            ServiceError::RegistrationError(msg) => ApiError::RegistrationError { field: msg },
            ServiceError::LoginError(msg) => ApiError::Unauthorized { field: msg },
            ServiceError::InternalError => ApiError::InternalServerError,
            ServiceError::InvalidToken(msg) => ApiError::Unauthorized { field: msg },
            ServiceError::InvalidRecipient(msg) => ApiError::BadRequest {
                field: ServiceError::InvalidRecipient(msg).to_string(),
            },
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let level = match self {
            ApiError::RegistrationError { field: _ } => Level::Warning,
            ApiError::Unauthorized { field: _ } => Level::Warning,
            ApiError::InternalServerError => Level::Error,
            ApiError::BadRequest { .. } => Level::Warning,
        };
        capture_message(&self.to_string(), level);

        let error_response = serde_json::json!({
            "error": self.to_string(),
            "field": match self {
                ApiError::BadRequest{field}=>Some(field),
                ApiError::RegistrationError { field } => Some(field),
                ApiError::Unauthorized { field } => Some(field),
                ApiError::InternalServerError => None,
            }
        });

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::RegistrationError { field: _ } => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized { field: _ } => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest { field: _ } => StatusCode::BAD_REQUEST,
        }
    }
}

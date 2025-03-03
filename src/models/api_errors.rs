use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

use crate::services::errors::ServiceError;

#[derive(Debug, Serialize, Display)]
pub enum ApiError {
    #[display("User already exist")]
    RegistrationError,

    #[display("Invalid credentials")]
    LoginError,

    #[display("An internal error occurred. Please try again later.")]
    InternalServerError,
}

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::DatabaseError(_e) => ApiError::InternalServerError,
            ServiceError::RegistrationError => ApiError::RegistrationError,
            ServiceError::LoginError => ApiError::LoginError,
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::RegistrationError => StatusCode::BAD_REQUEST,
            ApiError::LoginError => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

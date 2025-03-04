use crate::repositories::errors::RepositoryError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String),
    RegistrationError(String),
    LoginError(String),
    InternalError,
    InvalidToken(String),
    InvalidRecipient(String),
}

impl StdError for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::RegistrationError(msg) => write!(f, "User already exist: {}", msg),
            ServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ServiceError::LoginError(msg) => write!(f, "Registration error: {}", msg),
            ServiceError::InternalError => write!(f, "Internal error"),
            ServiceError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            ServiceError::InvalidRecipient(msg) => write!(f, "Invalid recipient: {}", msg),
        }
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::UserAlreadyExists(field) => ServiceError::RegistrationError(field),
            RepositoryError::DataNotFound(field) => ServiceError::LoginError(field),
            RepositoryError::DatabaseError(e) => ServiceError::DatabaseError(e.to_string()),
            // RepositoryError::DeserializationError(e) => ServiceError::DatabaseError(e.to_string()),
            // RepositoryError::SerializationError(e) => ServiceError::DatabaseError(e.to_string()),
        }
    }
}

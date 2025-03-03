use crate::repositories::errors::RepositoryError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String),
    RegistrationError,
    LoginError,
}

impl StdError for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::RegistrationError => write!(f, "User already exist"),
            ServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ServiceError::LoginError => write!(f, "Registration error"),
        }
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::UserAlreadyExists => ServiceError::RegistrationError,
            RepositoryError::DataNotFound(_field) => ServiceError::LoginError,
            RepositoryError::DatabaseError(e) => ServiceError::DatabaseError(e.to_string()),
            // RepositoryError::DeserializationError(e) => ServiceError::DatabaseError(e.to_string()),
            // RepositoryError::SerializationError(e) => ServiceError::DatabaseError(e.to_string()),
        }
    }
}

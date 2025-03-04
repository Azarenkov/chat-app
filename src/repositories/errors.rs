use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum RepositoryError {
    UserAlreadyExists(String),
    DataNotFound(String),
    DatabaseError(mongodb::error::Error),
}

impl StdError for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::UserAlreadyExists(field) => {
                write!(f, "User already exists: {}", field)
            }
            RepositoryError::DataNotFound(field) => write!(f, "{} data not found", field),
            RepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl From<mongodb::error::Error> for RepositoryError {
    fn from(err: mongodb::error::Error) -> Self {
        RepositoryError::DatabaseError(err)
    }
}

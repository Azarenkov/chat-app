use crate::{models::user::User, repositories::errors::RepositoryError};

pub trait UserRepositoryAbstract {
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find(&self, login: &str) -> Result<(), RepositoryError>;
}

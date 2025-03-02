use async_trait::async_trait;

use crate::{models::user::User, repositories::errors::RepositoryError};

use super::{auth_service_abstract::AuthServiceAbstract, errors::ServiceError};

#[async_trait]
pub trait UserRepositoryAbstract {
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find(&self, login: &str) -> Result<(), RepositoryError>;
}

pub struct AuthService {
    user_repository: Box<dyn UserRepositoryAbstract>,
}

impl AuthService {
    pub fn new(user_repository: Box<dyn UserRepositoryAbstract>) -> Self {
        Self { user_repository }
    }
}

impl AuthServiceAbstract for AuthService {
    async fn register(&self, user: &User) -> Result<(), ServiceError> {
        self.user_repository.find(&user.login).await?;
        self.user_repository.save(&user).await?;
        Ok(())
    }
}

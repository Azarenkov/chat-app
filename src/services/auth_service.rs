use async_trait::async_trait;

use crate::{models::user::User, repositories::errors::RepositoryError};

use super::{
    auth_service_abstract::AuthServiceAbstract, errors::ServiceError,
    jwt_service_abstract::JwtServiceAbstract,
};

#[async_trait]
pub trait UserRepositoryAbstract: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find(&self, login: &str) -> Result<(), RepositoryError>;
    async fn get(&self, user: &User) -> Result<User, RepositoryError>;
}

pub struct AuthService {
    user_repository: Box<dyn UserRepositoryAbstract>,
    jwt_service: Box<dyn JwtServiceAbstract>,
}

impl AuthService {
    pub fn new(
        user_repository: Box<dyn UserRepositoryAbstract>,
        jwt_service: Box<dyn JwtServiceAbstract>,
    ) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }
}

#[async_trait]
impl AuthServiceAbstract for AuthService {
    async fn register(&self, user: &User) -> Result<(), ServiceError> {
        self.user_repository.find(&user.login).await?;
        self.user_repository.save(&user).await?;
        Ok(())
    }

    async fn login(&self, user: &User) -> Result<String, ServiceError> {
        // self.user_repository.find(&user.login).await?;
        let db_user = self.user_repository.get(&user).await?;
        if user != &db_user {
            return Err(ServiceError::LoginError);
        }
        match self.jwt_service.generate_token(&user.login) {
            Ok(token) => Ok(token),
            Err(_) => Err(ServiceError::LoginError),
        }
    }
}

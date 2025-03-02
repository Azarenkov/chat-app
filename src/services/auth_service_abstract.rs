use async_trait::async_trait;

use crate::models::user::User;

use super::errors::ServiceError;

#[async_trait]
pub trait AuthServiceAbstract {
    async fn register(&self, user: &User) -> Result<(), ServiceError>;
}

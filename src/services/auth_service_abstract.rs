use crate::models::user::User;

use super::errors::ServiceError;

pub trait AuthServiceAbstract {
    async fn register(&self, user: &User) -> Result<(), ServiceError>;
}

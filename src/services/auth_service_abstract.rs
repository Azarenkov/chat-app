use crate::models::user::User;

pub trait AuthServiceAbstract {
    async fn register(&self, user: &User);
}

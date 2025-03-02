use mongodb::{
    bson::{doc, to_bson, Document},
    Collection,
};

use crate::{models::user::User, services::auth_service::UserRepositoryAbstract};

use super::errors::RepositoryError;

pub struct UserRepository {
    collection: Collection<Document>,
}

impl UserRepository {
    pub fn new(collection: Collection<Document>) -> Self {
        Self { collection }
    }
}

impl UserRepositoryAbstract for UserRepository {
    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        let doc = doc! {
            "$set": {"_id": &user.login, "password": &user.password }
        };
        // self.find(&user.login).await?;
        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn find(&self, login: &str) -> Result<(), RepositoryError> {
        let existing_token = self.collection.find_one(doc! {"_id": login}).await?;
        if existing_token.is_some() {
            return Err(RepositoryError::UserAlreadyExists);
        }
        Ok(())
    }
}

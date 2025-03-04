use std::sync::Arc;

use async_trait::async_trait;
use mongodb::{bson::doc, Collection, Database};

use crate::{models::user::User, services::auth_service::UserRepositoryAbstract};

use super::errors::RepositoryError;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            collection: database.collection("users"),
        }
    }
}

#[async_trait]
impl UserRepositoryAbstract for UserRepository {
    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        self.collection.insert_one(user).await?;
        Ok(())
    }

    async fn find(&self, login: &str) -> Result<(), RepositoryError> {
        let existing_login = self.collection.find_one(doc! {"_id": login}).await?;
        if existing_login.is_some() {
            return Err(RepositoryError::UserAlreadyExists(login.to_string()));
        }
        Ok(())
    }

    async fn get(&self, user: &User) -> Result<User, RepositoryError> {
        let doc = self.collection.find_one(doc! {"_id": &user.login}).await?;

        match doc {
            Some(user) => Ok(user),
            None => Err(RepositoryError::DataNotFound(user.login.to_string())),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct User {
    pub login: String,
    pub password: String,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(rename = "_id", alias = "login")]
    pub login: String,
    pub password: String,
}

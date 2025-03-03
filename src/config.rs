use std::{env, error::Error};

pub struct Config {
    pub mongo_uri: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Config {
            mongo_uri: env::var("MONGODB_URI")?,
            jwt_secret: env::var("JWT_SECRET")?,
        })
    }
}

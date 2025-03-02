use std::{env, error::Error};

pub struct Config {
    pub mongo_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Config {
            mongo_uri: env::var("MONGODB_URI")?,
        })
    }
}

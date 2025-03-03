use std::error::Error;

pub trait JwtServiceAbstract: Send + Sync {
    fn generate_token(&self, login: &str) -> Result<String, Box<dyn Error>>;
    fn validate_token(&self, token: &str) -> Result<String, Box<dyn Error>>;
}

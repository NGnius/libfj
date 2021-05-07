
use crate::robocraft::{DEFAULT_TOKEN};

/// Token generator for authenticated API endpoints
pub trait ITokenProvider {
    /// Retrieve the token to use
    fn token(&self) -> Result<String, ()>;
}

/// Token provider which uses DEFAULT_TOKEN
pub struct DefaultTokenProvider {
}

impl ITokenProvider for DefaultTokenProvider {
    fn token(&self) -> Result<String, ()> {
        Ok(DEFAULT_TOKEN.to_string())
    }
}

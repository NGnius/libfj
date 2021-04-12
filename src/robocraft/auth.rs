
use crate::robocraft::{DEFAULT_TOKEN};

pub trait ITokenProvider {
    fn token(&self) -> Result<String, ()>;
}

pub struct DefaultTokenProvider {
}

impl ITokenProvider for DefaultTokenProvider {
    fn token(&self) -> Result<String, ()> {
        Ok(DEFAULT_TOKEN.to_string())
    }
}

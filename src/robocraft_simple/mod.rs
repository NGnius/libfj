//! Simple, blocking Robocraft API

mod factory;
mod factory_request_builder;
pub use factory::{FactoryAPI};
pub use factory_request_builder::{FactorySearchBuilder};

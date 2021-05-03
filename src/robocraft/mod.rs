mod factory;
mod factory_json;
mod factory_request_builder;
pub use self::factory::{FactoryAPI, FACTORY_DOMAIN};
pub use self::factory_json::{FactoryInfo, FactoryRobotListInfo, RoboShopItemsInfo, FactoryRobotGetInfo};
pub use self::factory_request_builder::{FactorySearchBuilder, FactoryMovementType, FactoryOrderType, FactoryWeaponType, FactoryTextSearchType};
#[cfg(feature = "simple")]
pub(crate) use self::factory_json::{ListPayload};

mod auth;
pub use self::auth::{ITokenProvider, DefaultTokenProvider};

pub const DEFAULT_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJQdWJsaWNJZCI6IjEyMyIsIkRpc3BsYXlOYW1lIjoiVGVzdCIsIlJvYm9jcmFmdE5hbWUiOiJGYWtlQ1JGVXNlciIsIkZsYWdzIjpbXSwiaXNzIjoiRnJlZWphbSIsInN1YiI6IldlYiIsImlhdCI6MTU0NTIyMzczMiwiZXhwIjoyNTQ1MjIzNzkyfQ.ralLmxdMK9rVKPZxGng8luRIdbTflJ4YMJcd25dKlqg";

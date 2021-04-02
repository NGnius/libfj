mod client;

mod server;
mod server_json;
pub use self::server_json::{GameInfo, StatusInfo};
pub use self::server::{CLreServer};

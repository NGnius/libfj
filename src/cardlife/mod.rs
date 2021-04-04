mod client;

mod server;
mod server_json;
pub use self::server_json::{GameInfo, StatusInfo};
pub use self::server::{CLreServer};

mod live;
mod live_json;
pub use self::live::{LiveAPI};
pub use self::live_json::{AuthenticationInfo, LobbyInfo, LiveGameInfo};
pub(crate) use self::live_json::{AuthenticationPayload, LobbyPayload};

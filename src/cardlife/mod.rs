//! Cardlife vanilla and modded (CLre) APIs (WIP).
//! LiveAPI and CLreServer are mostly complete, but some other APIs are missing.

mod client;
mod client_json;
pub use client_json::ServerConfig;

mod server;
mod server_json;
pub use self::server_json::{GameInfo, StatusInfo};
pub use self::server::{CLreServer};

mod live;
mod live_json;
pub use self::live::{LiveAPI, AUTHENTICATION_DOMAIN, LOBBY_DOMAIN};
pub use self::live_json::{AuthenticationInfo, LobbyInfo, LiveGameInfo};
pub use self::live_json::{AuthenticationPayload, LobbyPayload};

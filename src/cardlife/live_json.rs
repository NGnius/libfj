use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthenticationPayload {
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SteamAuthenticationPayload {
    #[serde(rename = "SteamTicket")]
    pub steam_ticket: String,
    #[serde(rename = "Demo")]
    pub demo: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenPayload {
    #[serde(rename = "PublicID")] // [sic] compared to AuthenticationInfo's PublidId
    pub public_id: String,
    #[serde(rename = "Token")]
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TempGetUserIdPayload {
    #[serde(rename = "publicId")] // [sic] yay consistency
    pub public_id: String,
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TempGetUserIdResponse {
    #[serde(rename = "publicId")] // [sic] yay consistency
    pub public_id: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "userId")]
    pub user_id: i32,
}

/// Authentication information
#[derive(Deserialize, Serialize, Clone)]
pub struct AuthenticationInfo {
    /// User's public ID
    #[serde(rename = "PublicId")]
    pub public_id: String,
    /// User's email address
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// Account display name
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// Account purchases (???)
    #[serde(rename = "Purchases")]
    pub purchases: Vec<usize>, // ???
    /// Account flags (dev, admin, etc.???)
    #[serde(rename = "Flags")]
    pub flags: Vec<String>, // ???
    /// Is confirmed account?
    #[serde(rename = "Confirmed")]
    pub confirmed: bool,
    /// Temporary account token
    #[serde(rename = "Token")]
    pub token: String,
    /// Steam ID
    ///
    /// Since Steam users cannot be authenticated using this lib, this will always be blank or None
    #[serde(rename = "SteamId")]
    pub steam_id: Option<String>, // ???
    /// User ID
    #[serde(rename = "ID")]
    pub id: i32,
}

impl std::string::ToString for AuthenticationInfo {
    fn to_string(&self) -> String {
        format!("{} ({})", &self.display_name, &self.public_id)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LobbyPayload {
    #[serde(rename = "PublicId")]
    pub public_id: String,
}

/// Lobby information for available Cardlife servers
#[derive(Deserialize, Serialize, Clone)]
pub struct LobbyInfo {
    #[serde(rename = "Games")]
    /// Available servers' information
    pub games: Vec<LiveGameInfo>,
}

/// Server information for a single Cardlife server
#[derive(Deserialize, Serialize, Clone)]
pub struct LiveGameInfo {
    /// Server game ID
    #[serde(rename = "Id")]
    pub id: usize,
    /// World name
    #[serde(rename = "WorldName")]
    pub world_name: String,
    /// Max players
    #[serde(rename = "MaxPlayers")]
    pub max_players: usize,
    /// Current player count
    #[serde(rename = "CurrentPlayers")]
    pub current_players: usize,
    /// Server version
    #[serde(rename = "GameVersion")]
    pub game_version: String,
    /// Ping latency
    #[serde(rename = "Ping")]
    pub ping: usize,
    /// Account has already joined this server?
    #[serde(rename = "HasPlayed")]
    pub has_played: bool,
    /// Server is password protected?
    #[serde(rename = "HasPassword")]
    pub has_password: bool,
    /// PvP is enabled on this server?
    #[serde(rename = "IsPvp")]
    pub is_pvp: bool,
    /// EasyAntiCheat is enabled on this server?
    #[serde(rename = "IsAntiCheatEnabled")]
    pub is_anticheat_enabled: bool,
    /// Official server?
    #[serde(rename = "IsOfficial")]
    pub is_official: bool,
    /// Mods installed on this server
    #[serde(rename = "ModInfo")]
    pub mod_info: String,
    /// Server region
    #[serde(rename = "Region")]
    pub region: String,
}

impl std::string::ToString for LiveGameInfo {
    fn to_string(&self) -> String {
        format!("{} ({}):{}/{}", self.world_name, self.id, self.current_players, self.max_players)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct AuthenticationPayload {
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthenticationInfo {
    #[serde(rename = "PublicId")]
    pub public_id: String,
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Purchases")]
    purchases: Vec<String>, // ???
    #[serde(rename = "Flags")]
    flags: Vec<String>, // ???
    #[serde(rename = "Confirmed")]
    pub confirmed: bool,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "SteamId")]
    steam_id: Option<String>, // ???
    #[serde(rename = "ID")]
    pub id: usize,
}

impl std::string::ToString for AuthenticationInfo {
    fn to_string(&self) -> String {
        format!("{} ({})", &self.display_name, &self.public_id)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct LobbyPayload {
    #[serde(rename = "PublicId")]
    pub public_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LobbyInfo {
    #[serde(rename = "Games")]
    pub games: Vec<LiveGameInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LiveGameInfo {
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "WorldName")]
    pub world_name: String,
    #[serde(rename = "MaxPlayers")]
    pub max_players: usize,
    #[serde(rename = "CurrentPlayers")]
    pub current_players: usize,
    #[serde(rename = "GameVersion")]
    pub game_version: String,
    #[serde(rename = "Ping")]
    pub ping: usize,
    #[serde(rename = "HasPlayed")]
    pub has_played: bool,
    #[serde(rename = "HasPassword")]
    pub has_password: bool,
    #[serde(rename = "IsPvp")]
    pub is_pvp: bool,
    #[serde(rename = "IsAntiCheatEnabled")]
    pub is_anticheat_enabled: bool,
    #[serde(rename = "IsOfficial")]
    pub is_official: bool,
    #[serde(rename = "ModInfo")]
    pub mod_info: String,
    #[serde(rename = "Region")]
    pub region: String,
}

impl std::string::ToString for LiveGameInfo {
    fn to_string(&self) -> String {
        format!("{} ({}):{}/{}", self.world_name, self.id, self.current_players, self.max_players)
    }
}
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct GameInfo {
    #[serde(rename = "MaxPlayers")]
    pub max_players: usize,
    #[serde(rename = "GameId")]
    pub game_id: usize,
    #[serde(rename = "GameGuid")]
    pub game_guid: String,
    #[serde(rename = "WorldName")]
    pub world_name: String,
    #[serde(rename = "GameHostType")]
    pub game_host_type: usize,
    #[serde(rename = "PvP")]
    pub pvp: bool,
    #[serde(rename = "PhotonRegionOverride")]
    pub photon_region_override: String,
    #[serde(rename = "ServerPassword")]
    pub server_password: String,
    #[serde(rename = "AdminPassword")]
    pub admin_password: String,
}

impl std::string::ToString for GameInfo {
    fn to_string(&self) -> String {
        format!("{} ({})", self.world_name, self.game_guid)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct StatusInfo {
    #[serde(rename = "PlayersMax")]
    pub max_players: usize,
    #[serde(rename = "PlayerCount")]
    pub player_count: usize,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "OnlinePlayers")]
    pub online_players: Vec<PlayerStatusInfo>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PlayerStatusInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isDev")]
    pub is_dev: bool,
    #[serde(rename = "x")]
    pub x: f32,
    #[serde(rename = "y")]
    pub y: f32,
    #[serde(rename = "z")]
    pub z: f32,
}

impl std::string::ToString for PlayerStatusInfo {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }
}

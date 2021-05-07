use serde::{Deserialize, Serialize};

/// CLre game info
#[derive(Deserialize, Serialize, Clone)]
pub struct GameInfo {
    /// Max allowed player count
    #[serde(rename = "MaxPlayers")]
    pub max_players: usize,
    /// Server world ID
    #[serde(rename = "GameId")]
    pub game_id: usize,
    /// Server world GUID
    #[serde(rename = "GameGuid")]
    pub game_guid: String,
    /// World name
    #[serde(rename = "WorldName")]
    pub world_name: String,
    /// Game host type
    #[serde(rename = "GameHostType")]
    pub game_host_type: usize,
    /// Is PvP enabled?
    #[serde(rename = "PvP")]
    pub pvp: bool,
    /// Photon server region override
    #[serde(rename = "PhotonRegionOverride")]
    pub photon_region_override: String,
    /// Server password
    #[serde(rename = "ServerPassword")]
    pub server_password: String,
    /// Admin priviledge password
    #[serde(rename = "AdminPassword")]
    pub admin_password: String,
}

impl std::string::ToString for GameInfo {
    fn to_string(&self) -> String {
        format!("{} ({})", &self.world_name, &self.game_guid)
    }
}

/// CLre_server status information
#[derive(Deserialize, Serialize, Clone)]
pub struct StatusInfo {
    /// Maximum player count
    #[serde(rename = "PlayersMax")]
    pub max_players: usize,
    /// Current player count
    #[serde(rename = "PlayerCount")]
    pub player_count: usize,
    /// Server status (enum as string)
    #[serde(rename = "Status")]
    pub status: String,
    /// Information on all online players in this server
    #[serde(rename = "OnlinePlayers")]
    pub online_players: Vec<PlayerStatusInfo>
}

/// A single online player's information
#[derive(Deserialize, Serialize, Clone)]
pub struct PlayerStatusInfo {
    /// Player public ID
    #[serde(rename = "id")]
    pub id: String,
    /// Player name
    #[serde(rename = "name")]
    pub name: String,
    /// Is the player a developer?
    #[serde(rename = "isDev")]
    pub is_dev: bool,
    /// Player's location on x-axis
    #[serde(rename = "x")]
    pub x: f32,
    /// Player's location on y-axis
    #[serde(rename = "y")]
    pub y: f32,
    /// Player's location on z-axis
    #[serde(rename = "z")]
    pub z: f32,
}

impl std::string::ToString for PlayerStatusInfo {
    fn to_string(&self) -> String {
        format!("{} ({})", &self.name, &self.id)
    }
}

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// ServerConfig.json format in the root of the game files
#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    #[serde(rename = "AuthUrl")]
    pub auth_url: String,
    #[serde(rename = "LobbyUrl")]
    pub lobby_url: String,
    #[serde(rename = "InventoryUrl")]
    pub inventory_url: String,
    #[serde(rename = "FallbackAuthUrl")]
    pub fallback_auth_url: String,
    #[serde(rename = "FallbackLobbyUrl")]
    pub fallback_lobby_url: String,
    #[serde(rename = "FallbackInventoryUrl")]
    pub fallback_inventory_url: String,
    #[serde(rename = "GameServerPath")]
    pub game_server_path: PathBuf,
    #[serde(rename = "GameServerExe")]
    pub game_server_exe: PathBuf,
    #[serde(rename = "PhotonUrl")]
    pub photon_url: String,
}

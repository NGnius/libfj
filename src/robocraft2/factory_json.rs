use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorPayload {
    #[serde(rename = "error")]
    pub error: isize,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

// search endpoint

#[derive(Deserialize, Serialize, Clone)]
pub struct SearchPayload {
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "baseCpuMinimum")]
    pub base_minimum_cpu: Option<isize>,
    #[serde(rename = "baseCpuMaximum")]
    pub base_maximum_cpu: Option<isize>,
    #[serde(rename = "weaponCpuMinimum")]
    pub weapon_minimum_cpu: Option<isize>,
    #[serde(rename = "weaponCpuMaximum")]
    pub weapon_maximum_cpu: Option<isize>,
    #[serde(rename = "cosmeticCpuMinimum")]
    pub cosmetic_minimum_cpu: Option<isize>,
    #[serde(rename = "cosmeticCpuMaximum")]
    pub cosmetic_maximum_cpu: Option<isize>,
    #[serde(rename = "clusterMinimum")]
    pub cluster_minimum: Option<isize>,
    #[serde(rename = "clusterMaximum")]
    pub cluster_maximum: Option<isize>,
    #[serde(rename = "dateMinimum")]
    pub date_minimum: Option<String>,
    #[serde(rename = "dateMaximum")]
    pub date_maximum: Option<String>,
    #[serde(rename = "creatorId")]
    pub creator_id: Option<String>, // GUID
    #[serde(rename = "page")]
    pub page: Option<isize>,
    #[serde(rename = "count")]
    pub count: Option<isize>,
    #[serde(rename = "sortBy")]
    pub sort_by: String,
    #[serde(rename = "orderBy")]
    pub order_by: String,
}

impl Default for SearchPayload {
    fn default() -> Self {
        Self {
            text: None,
            base_minimum_cpu: None,
            base_maximum_cpu: None,
            weapon_minimum_cpu: None,
            weapon_maximum_cpu: None,
            cosmetic_minimum_cpu: None,
            cosmetic_maximum_cpu: None,
            cluster_minimum: None,
            cluster_maximum: None,
            date_minimum: None,
            date_maximum: None,
            creator_id: None,
            page: None,
            count: None,
            sort_by: "default".to_owned(),
            order_by: "ascending".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchResponse {
    #[serde(rename = "results")]
    pub results: Vec<SearchResponseItem>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchResponseItem {
    #[serde(rename = "robot")]
    pub robot: RobotInfo,
    #[serde(rename = "prices")]
    pub prices: Vec<RobotPrice>,
    #[serde(rename = "purchased")]
    pub purchased: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RobotInfo {
    #[serde(rename = "id")]
    pub id: String, // GUID
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String, // GUID
    #[serde(rename = "creatorName")]
    pub creator_name: String,
    #[serde(rename = "created")]
    pub created: String, // date
    #[serde(rename = "image")]
    pub image: String, // base64??
    #[serde(rename = "baseCpu")]
    pub base_cpu: isize,
    #[serde(rename = "weaponCpu")]
    pub weapon_cpu: isize,
    #[serde(rename = "cosmeticCpu")]
    pub cosmetic_cpu: isize,
    #[serde(rename = "clusterCount")]
    pub cluster_count: isize,
    #[serde(rename = "blockCounts")]
    pub block_counts: std::collections::HashMap<usize, usize>,
    #[serde(rename = "materialsUsed")]
    pub materials_used: std::collections::HashSet<isize>,
    #[serde(rename = "minimumOffsetX")]
    pub minimum_offset_x: f64,
    #[serde(rename = "minimumOffsetY")]
    pub minimum_offset_y: f64,
    #[serde(rename = "minimumOffsetZ")]
    pub minimum_offset_z: f64,
    #[serde(rename = "maximumOffsetX")]
    pub maximum_offset_x: f64,
    #[serde(rename = "maximumOffsetY")]
    pub maximum_offset_y: f64,
    #[serde(rename = "maximumOffsetZ")]
    pub maximum_offset_z: f64,
}

impl std::string::ToString for RobotInfo {
    fn to_string(&self) -> String {
        format!("{} ({}) by {} ({})", &self.name, &self.id, &self.creator_name, &self.creator_id)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RobotPrice {
    #[serde(rename = "currency")]
    pub currency: isize,
    #[serde(rename = "amount")]
    pub amount: isize,
}

// create robot endpoint

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateRobotPayload {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "data")]
    pub data: String, // base64
    #[serde(rename = "image")]
    pub image: String, // base64??
    #[serde(rename = "baseCpu")]
    pub base_cpu: isize,
    #[serde(rename = "weaponCpu")]
    pub weapon_cpu: isize,
    #[serde(rename = "cosmeticCpu")]
    pub cosmetic_cpu: isize,
    #[serde(rename = "clusterCount")]
    pub cluster_count: isize,
    #[serde(rename = "blockCounts")]
    pub block_counts: std::collections::HashMap<usize, usize>,
    #[serde(rename = "materialsUsed")]
    pub materials_used: std::collections::HashSet<isize>,
    #[serde(rename = "minimumOffsetX")]
    pub minimum_offset_x: f64,
    #[serde(rename = "minimumOffsetY")]
    pub minimum_offset_y: f64,
    #[serde(rename = "minimumOffsetZ")]
    pub minimum_offset_z: f64,
    #[serde(rename = "maximumOffsetX")]
    pub maximum_offset_x: f64,
    #[serde(rename = "maximumOffsetY")]
    pub maximum_offset_y: f64,
    #[serde(rename = "maximumOffsetZ")]
    pub maximum_offset_z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateRobotResponse {
    #[serde(rename = "header")]
    pub header: RobotInfo,
}

// factory info endpoint

// (no payload -- this is a GET request)

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FactoryInfoResponse {
    #[serde(rename = "robotCount")]
    pub robot_count: isize,
    #[serde(rename = "robotLimit")]
    pub robot_limit: isize,
    #[serde(rename = "publishedRobotCount")]
    pub published_robot_count: isize,
    #[serde(rename = "publishedRobotLimit")]
    pub published_robot_limit: isize,
}

// publish robot endpoint

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PublishRobotPayload {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "techPoints")]
    pub techpoints: isize,
    #[serde(rename = "bloxCoin")]
    pub bloxcoin: isize,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PublishRobotResponse {
    #[serde(rename = "header")]
    pub header: RobotInfo,
    #[serde(rename = "data")]
    pub data: String, // base64
}

// get my robots endpoint

// (no payload -- this is a GET request)

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MyRobotsResponse {
    #[serde(rename = "vehicles")]
    pub vehicles: Vec<RobotInfo>,
}

// get robot endpoint

// (no payload -- this is a GET request)

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetRobotResponse {
    #[serde(rename = "header")]
    pub header: RobotInfo,
    #[serde(rename = "data")]
    pub data: String, // base64
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "created")]
    pub created: String, // date
}

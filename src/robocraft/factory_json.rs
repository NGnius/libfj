use serde::{Deserialize, Serialize};

// list endpoint

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct ListPayload {
    #[serde(rename = "page")]
    pub page: isize,
    #[serde(rename = "pageSize")]
    pub page_size: isize,
    #[serde(rename = "order")]
    pub order: isize,
    #[serde(rename = "playerFilter")]
    pub player_filter: bool,
    #[serde(rename = "movementFilter")]
    pub movement_filter: String, // csv int enums as str
    #[serde(rename = "movementCategoryFilter")]
    pub movement_category_filter: String, // csv int enums as str
    #[serde(rename = "weaponFilter")]
    pub weapon_filter: String, // csv int enums as str
    #[serde(rename = "weaponCategoryFilter")]
    pub weapon_category_filter: String, // csv int enums as str
    #[serde(rename = "minimumCpu")]
    pub minimum_cpu: isize,
    #[serde(rename = "maximumCpu")]
    pub maximum_cpu: isize,
    #[serde(rename = "textFilter")]
    pub text_filter: String,
    #[serde(rename = "textSearchField")]
    pub text_search_field: isize, // ???
    #[serde(rename = "buyable")]
    pub buyable: bool,
    #[serde(rename = "prependFeaturedRobot")]
    pub prepend_featured_robot: bool,
    #[serde(rename = "featuredOnly")]
    pub featured_only: bool,
    #[serde(rename = "defaultPage")]
    pub default_page: bool,
}

impl ListPayload {
    pub fn default() -> ListPayload {
        ListPayload {
            page: 1,
            page_size: 100,
            order: 0,
            player_filter: false,
            movement_filter: "100000,200000,300000,400000,500000,600000,700000,800000,900000,1000000,1100000,1200000".to_string(),
            movement_category_filter: "100000,200000,300000,400000,500000,600000,700000,800000,900000,1000000,1100000,1200000".to_string(),
            weapon_filter: "10000000,20000000,25000000,30000000,40000000,50000000,60000000,65000000,70100000,75000000".to_string(),
            weapon_category_filter: "10000000,20000000,25000000,30000000,40000000,50000000,60000000,65000000,70100000,75000000".to_string(),
            minimum_cpu: -1,
            maximum_cpu: -1,
            text_filter: "".to_string(),
            text_search_field: 0,
            buyable: true,
            prepend_featured_robot: false,
            featured_only: false,
            default_page: true,
        }
    }
    
    pub fn empty() -> ListPayload {
        ListPayload {
            page: 1,
            page_size: 100,
            order: 0,
            player_filter: false,
            movement_filter: "".to_string(),
            movement_category_filter: "".to_string(),
            weapon_filter: "".to_string(),
            weapon_category_filter: "".to_string(),
            minimum_cpu: -1,
            maximum_cpu: -1,
            text_filter: "".to_string(),
            text_search_field: 0,
            buyable: true,
            prepend_featured_robot: false,
            featured_only: false,
            default_page: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FactoryInfo<T> {
    #[serde(rename = "response")]
    pub response: T,
    #[serde(rename = "statusCode")]
    pub status_code: usize,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RoboShopItemsInfo {
    #[serde(rename = "roboShopItems")]
    pub roboshop_items: Vec<FactoryRobotListInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FactoryRobotListInfo {
    #[serde(rename = "itemId")]
    pub item_id: usize,
    #[serde(rename = "itemName")]
    pub item_name: String,
    #[serde(rename = "itemDescription")]
    pub item_description: String,
    #[serde(rename = "thumbnail")]
    pub thumbnail: String, // url
    #[serde(rename = "addedBy")]
    pub added_by: String,
    #[serde(rename = "addedByDisplayName")]
    pub added_by_display_name: String,
    #[serde(rename = "addedDate")]
    pub added_date: String, // ISO date
    #[serde(rename = "expiryDate")]
    pub expiry_date: String, // ISO date
    #[serde(rename = "cpu")]
    pub cpu: usize,
    #[serde(rename = "totalRobotRanking")]
    pub total_robot_ranking: usize,
    #[serde(rename = "rentCount")]
    pub rent_count: usize,
    #[serde(rename = "buyCount")]
    pub buy_count: usize,
    #[serde(rename = "buyable")]
    pub buyable: bool,
    #[serde(rename = "removedDate")]
    pub removed_date: Option<String>,
    #[serde(rename = "banDate")]
    pub ban_date: Option<String>,
    #[serde(rename = "featured")]
    pub featured: bool,
    #[serde(rename = "bannerMessage")]
    pub banner_message: Option<String>,
    #[serde(rename = "combatRating")]
    pub combat_rating: f32,
    #[serde(rename = "cosmeticRating")]
    pub cosmetic_rating: f32,
    #[serde(rename = "cubeAmounts")]
    pub cube_amounts: String, // JSON as str
}

impl std::string::ToString for FactoryRobotListInfo {
    fn to_string(&self) -> String {
        format!("{} by {} ({})", &self.item_name, &self.added_by_display_name, &self.item_id)
    }
}

// get/<item_id> endpoint

#[derive(Deserialize, Serialize, Clone)]
pub struct FactoryRobotGetInfo {
    #[serde(rename = "id")]
    pub item_id: usize,
    #[serde(rename = "name")]
    pub item_name: String,
    #[serde(rename = "description")]
    pub item_description: String,
    #[serde(rename = "thumbnail")]
    pub thumbnail: String, // url
    #[serde(rename = "addedBy")]
    pub added_by: String,
    #[serde(rename = "addedByDisplayName")]
    pub added_by_display_name: String,
    #[serde(rename = "addedDate")]
    pub added_date: String, // ISO date
    #[serde(rename = "expiryDate")]
    pub expiry_date: String, // ISO date
    #[serde(rename = "cpu")]
    pub cpu: usize,
    #[serde(rename = "totalRobotRanking")]
    pub total_robot_ranking: usize,
    #[serde(rename = "rentCount")]
    pub rent_count: usize,
    #[serde(rename = "buyCount")]
    pub buy_count: usize,
    #[serde(rename = "buyable")]
    pub buyable: bool,
    #[serde(rename = "removedDate")]
    pub removed_date: Option<String>,
    #[serde(rename = "banDate")]
    pub ban_date: Option<String>,
    #[serde(rename = "featured")]
    pub featured: bool,
    #[serde(rename = "bannerMessage")]
    pub banner_message: Option<String>,
    #[serde(rename = "combatRating")]
    pub combat_rating: f32,
    #[serde(rename = "cosmeticRating")]
    pub cosmetic_rating: f32,
    #[serde(rename = "cubeData")]
    pub cube_data: String,
    #[serde(rename = "colourData")]
    pub colour_data: String,
    #[serde(rename = "cubeAmounts")]
    pub cube_amounts: String, // JSON as str
}

impl std::string::ToString for FactoryRobotGetInfo {
    fn to_string(&self) -> String {
        format!("{} by {} ({})", &self.item_name, &self.added_by_display_name, &self.item_id)
    }
}

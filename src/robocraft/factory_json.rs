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


/// Standard factory response format.
#[derive(Deserialize, Serialize, Clone)]
pub struct FactoryInfo<T> {
    #[serde(rename = "response")]
    /// The response data
    pub response: T,
    #[serde(rename = "statusCode")]
    /// HTTP status code for the query
    pub status_code: usize,
}

/// Collection of robots in response to a list query.
#[derive(Deserialize, Serialize, Clone)]
pub struct RoboShopItemsInfo {
    #[serde(rename = "roboShopItems")]
    /// Robot items
    pub roboshop_items: Vec<FactoryRobotListInfo>,
}

/// Information about a single robot in response to a list query.
///
/// This does not include robot block data since it is not returned by this API endpoint.
/// Use `FactoryAPI.get(data.item_id)` to retrieve all info for a single robot.
#[derive(Deserialize, Serialize, Clone)]
pub struct FactoryRobotListInfo {
    /// Item ID
    #[serde(rename = "itemId")]
    pub item_id: usize,
    /// Robot name
    #[serde(rename = "itemName")]
    pub item_name: String,
    /// Robot description
    #[serde(rename = "itemDescription")]
    pub item_description: String,
    /// Thumbnail URL, as displayed to preview the robot.
    #[serde(rename = "thumbnail")]
    pub thumbnail: String, // url
    /// Robot author's username or UUID
    #[serde(rename = "addedBy")]
    pub added_by: String,
    /// Robot author's display name
    #[serde(rename = "addedByDisplayName")]
    pub added_by_display_name: String,
    /// Date added, in standard ISO format
    #[serde(rename = "addedDate")]
    pub added_date: String, // ISO date
    /// Expiry date, in standard ISO format
    #[serde(rename = "expiryDate")]
    pub expiry_date: String, // ISO date
    /// Robot CPU value
    #[serde(rename = "cpu")]
    pub cpu: usize,
    /// Robot RR
    #[serde(rename = "totalRobotRanking")]
    pub total_robot_ranking: usize,
    /// Robot's rentals
    #[serde(rename = "rentCount")]
    pub rent_count: usize,
    /// Robot's purchases
    #[serde(rename = "buyCount")]
    pub buy_count: usize,
    /// Is this robot buyable? (probably yes, unless you're a mod/admin or missing parts)
    #[serde(rename = "buyable")]
    pub buyable: bool,
    /// Removed date, in standard ISO format (probably None, unless authenticated as a mod/admin)
    #[serde(rename = "removedDate")]
    pub removed_date: Option<String>,
    /// Author ban date, in standard ISO format (probable None)
    #[serde(rename = "banDate")]
    pub ban_date: Option<String>,
    /// Is this robot featured?
    #[serde(rename = "featured")]
    pub featured: bool,
    /// CRF Banner message
    #[serde(rename = "bannerMessage")]
    pub banner_message: Option<String>,
    /// Robot's combat rating, out of 5
    #[serde(rename = "combatRating")]
    pub combat_rating: f32,
    /// Robot's cosmetic rating, out of 5
    #[serde(rename = "cosmeticRating")]
    pub cosmetic_rating: f32,
    /// Robot's count of (some?) blocks it uses
    #[serde(rename = "cubeAmounts")]
    pub cube_amounts: String, // JSON as str
}

impl std::string::ToString for FactoryRobotListInfo {
    fn to_string(&self) -> String {
        format!("{} by {} ({})", &self.item_name, &self.added_by_display_name, &self.item_id)
    }
}

// get/<item_id> endpoint
/// Complete information about a single robot in response to a get query.
/// Please refer to FactoryRobotListInfo for more in-depth documentation of fields.
#[derive(Deserialize, Serialize, Clone)]
pub struct FactoryRobotGetInfo {
    /// Item ID
    #[serde(rename = "id")]
    pub item_id: usize,
    /// Robot name
    #[serde(rename = "name")]
    pub item_name: String,
    /// Robot description
    #[serde(rename = "description")]
    pub item_description: String,
    /// Robot thumbnail URL
    #[serde(rename = "thumbnail")]
    pub thumbnail: String, // url
    /// Robot author's username or UUID
    #[serde(rename = "addedBy")]
    pub added_by: String,
    /// Robot author's display name
    #[serde(rename = "addedByDisplayName")]
    pub added_by_display_name: String,
    /// ISO date added
    #[serde(rename = "addedDate")]
    pub added_date: String, // ISO date
    /// ISO date expiring
    #[serde(rename = "expiryDate")]
    pub expiry_date: String, // ISO date
    /// CPU
    #[serde(rename = "cpu")]
    pub cpu: usize,
    /// RR
    #[serde(rename = "totalRobotRanking")]
    pub total_robot_ranking: usize,
    /// Robot rent count
    #[serde(rename = "rentCount")]
    pub rent_count: usize,
    /// Robot buy count
    #[serde(rename = "buyCount")]
    pub buy_count: usize,
    /// Robot is buyable?
    #[serde(rename = "buyable")]
    pub buyable: bool,
    /// ISO date removed
    #[serde(rename = "removedDate")]
    pub removed_date: Option<String>,
    /// ISO date banned
    #[serde(rename = "banDate")]
    pub ban_date: Option<String>,
    /// Robot is featured?
    #[serde(rename = "featured")]
    pub featured: bool,
    /// CRF banner message
    #[serde(rename = "bannerMessage")]
    pub banner_message: Option<String>,
    /// Robot's combat rating, out of 5
    #[serde(rename = "combatRating")]
    pub combat_rating: f32,
    /// Robot's cosmetic rating, out of 5
    #[serde(rename = "cosmeticRating")]
    pub cosmetic_rating: f32,
    /// Robot block cube and position data
    #[serde(rename = "cubeData")]
    pub cube_data: String,
    /// Robot block colour data
    #[serde(rename = "colourData")]
    pub colour_data: String,
    /// Cube counts
    #[serde(rename = "cubeAmounts")]
    pub cube_amounts: String, // JSON as str
}

impl std::string::ToString for FactoryRobotGetInfo {
    fn to_string(&self) -> String {
        format!("{} by {} ({})", &self.item_name, &self.added_by_display_name, &self.item_id)
    }
}

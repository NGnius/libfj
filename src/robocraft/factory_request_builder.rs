use reqwest::{RequestBuilder, Error};
use num_enum::{TryFromPrimitive};

use crate::robocraft::{FactoryInfo, RoboShopItemsInfo};
use crate::robocraft::factory_json::ListPayload;

/// Factory list response ordering
#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum FactoryOrderType {
    /// Suggested (default)
    Suggested = 0,
    /// Combat rating (decreasing?)
    CombatRating = 1,
    /// Cosmetic rating (decreasing?)
    CosmeticRating = 2,
    /// Date added (oldest first?)
    Added = 3,
    /// CPU value (decreasing?)
    CPU = 4,
    /// Purchases (decreasing)
    MostBought = 5,
}

/// Robot movement categories
#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum FactoryMovementType {
    /// Vrooooom
    Wheels = 100000,
    /// Woooooosh
    Hovers = 200000,
    /// Fwoooosh
    Aerofoils=300000,
    /// Also fwoooosh (but actually a different movement type, trust me)
    Thrusters=400000,
    /// Also also fwoooosh (but also a different movement type)
    Rudders=500000,
    /// Ewwww
    InsectLegs=600000,
    /// Mechs are cool
    MechLegs=700000,
    /// Skis and turning skis
    Skis=800000,
    /// All tank treads
    TankTreads=900000,
    /// Wrrrrrrrrrr
    Rotors=1000000,
    /// Mech legs, but faster
    Sprinters=1100000,
    /// Wrrrrr but for Fwoooosh
    Propellers=1200000
}

/// Robot weapon categories
#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum FactoryWeaponType {
    /// All laser weapons (aka Lasor, SMG)
    Laser=10000000,
    /// All plasma launcher weapons
    PlasmaLauncher=20000000,
    /// Mortar
    GyroMortar=25000000,
    /// All rails
    RailCannon=30000000,
    /// All healing weapons
    NanoDisruptor=40000000,
    /// All tesla blade melee weapons
    TeslaBlade=50000000,
    /// All aeroflak weapons
    AeroflakCannon=60000000,
    /// All shotgun weapons
    IonCannon=65000000,
    /// Lol
    ProtoSeeker=70100000,
    /// All chain weapons
    ChainShredder=75000000,
}

/// Text field search modes
#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum FactoryTextSearchType {
    /// Search players and robot names
    All=0,
    /// Search players only
    Player=1,
    /// Search robot names only
    Name=2,
}

/// Factory API list query builder
pub struct FactorySearchBuilder {
    reqwest_builder: RequestBuilder,
    payload: ListPayload,
    token: Option<String>,
}

impl FactorySearchBuilder {
    pub(crate) fn new(request_builder: RequestBuilder, token: Option<String>) -> FactorySearchBuilder {
        FactorySearchBuilder {
            reqwest_builder: request_builder,
            payload: ListPayload::empty(),
            token,
        }
    }
    
    /// Retrieve list page page_number
    pub fn page(mut self, page_number: isize) -> Self {
        self.payload.page = page_number;
        self
    }
    
    /// Retrieve page_size items per page (this is unreliable)
    pub fn items_per_page(mut self, page_size: isize) -> Self {
        self.payload.page_size = page_size;
        self
    }
    
    /// Order list by order_type
    pub fn order(mut self, order_type: FactoryOrderType) -> Self {
        self.payload.order = order_type as isize;
        self
    }
    
    /* // this appears to not do anything (removed to prevent confusion)
    // use text_search_type(FactoryTextSearchType::Player) instead
    pub fn players_only(mut self, p: bool) -> Self {
        self.payload.player_filter = p;
        self
    }
    */
    
    /// Retrieve items with movement type.
    ///
    /// Multiple calls to this function will cause logical OR behaviour.
    /// e.g. results will contain robots with Wheels OR Aerofoils (or both).
    pub fn movement_or(mut self, movement_type: FactoryMovementType) -> Self {
        if self.payload.movement_filter == "" {
            self.payload.movement_filter = format!("{},{}", &self.payload.movement_filter, movement_type as isize);
        } else {
            self.payload.movement_filter = (movement_type as isize).to_string();
        }
        self.payload.movement_category_filter = self.payload.movement_filter.clone();
        self
    }
    
    /// Override allowed movement types
    pub fn movement_raw(mut self, filter: String) -> Self {
        self.payload.movement_filter = filter.clone();
        self.payload.movement_category_filter = filter.clone();
        self
    }
    
    /// Retrieve items with weapon type.
    ///
    /// Multiple calls to this function will cause logical OR behaviour.
    /// e.g. results will contain robots with ChainShredder OR GyroMortar (or both).
    pub fn weapon_or(mut self, weapon_type: FactoryWeaponType) -> Self {
        if self.payload.weapon_filter == "" {
            self.payload.weapon_filter = format!("{},{}", &self.payload.weapon_filter, weapon_type as isize);
        } else {
            self.payload.weapon_filter = (weapon_type as isize).to_string();
        }
        self.payload.weapon_category_filter = self.payload.weapon_filter.clone();
        self
    }
    
    /// Override allowed weapon types
    pub fn weapon_raw(mut self, filter: String) -> Self {
        self.payload.weapon_filter = filter.clone();
        self.payload.weapon_category_filter = filter.clone();
        self
    }
    
    /// Retrieve items within the specified CPU min and max values
    pub fn cpu_range(mut self, min: isize, max: isize) -> Self {
        self.payload.minimum_cpu = min;
        self.payload.maximum_cpu = max;
        self
    }
    
    /// Retrieve items with CPU no lower than min
    /// overrides cpu_range()
    pub fn min_cpu(mut self, min: isize) -> Self {
        self.payload.minimum_cpu = min;
        self
    }
    
    /// Retrieve items with CPU no greater than max
    /// overrides cpu_range()
    pub fn max_cpu(mut self, max: isize) -> Self {
        self.payload.maximum_cpu = max;
        self
    }
    
    /// Retrieve items with any minimum CPU
    pub fn no_minimum_cpu(mut self) -> Self {
        self.payload.minimum_cpu = -1;
        self
    }
    
    /// Retrieve items with any maximum CPU
    pub fn no_maximum_cpu(mut self) -> Self {
        self.payload.maximum_cpu = -1;
        self
    }
    
    /// Retrieve items which match text
    pub fn text(mut self, t: String) -> Self {
        self.payload.text_filter = t;
        self
    }
    
    /// Text filter searches search_type
    pub fn text_search_type(mut self, search_type: FactoryTextSearchType) -> Self {
        self.payload.text_search_field = search_type as isize;
        self
    }
    
    // setting buyable to false while using the default token provider will cause HTTP status 500 error
    /// Retrieve only items which are buyable for current account? (default: false)
    /// Buyable means that the account owns all blocks required.
    /// This will cause an error when using DEFAULT_TOKEN
    pub fn buyable(mut self, b: bool) -> Self {
        self.payload.buyable = b;
        self
    }
    
    /// Retrieve items with featured robot at start? (default: false)
    pub fn prepend_featured(mut self, b: bool) -> Self {
        self.payload.prepend_featured_robot = b;
        self
    }
    
    /// Retrieve default robot list? (default: false)
    /// The default page is the CRF landing page (I think?)
    pub fn default_page(mut self, b: bool) -> Self {
        self.payload.default_page = b;
        self
    }
    
    /// Execute list query
    pub async fn send(mut self) -> Result<FactoryInfo<RoboShopItemsInfo>, Error> {
        self.reqwest_builder = self.reqwest_builder.json(&self.payload);
        if let Some(token) = self.token.clone() {
            self.reqwest_builder = self.reqwest_builder.header("Authorization", "Web ".to_owned() + &token);
        }
        let result = self.reqwest_builder.send().await;
        //dbg!(&result);
        match result {
            Ok(response) => {
                response.error_for_status()?
                    .json::<FactoryInfo<RoboShopItemsInfo>>().await
            }
            Err(e) => Err(e),
        }
    }
}

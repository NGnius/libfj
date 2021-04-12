use reqwest::{RequestBuilder, Error};

use crate::robocraft::{FactoryInfo};
use crate::robocraft::factory_json::ListPayload;

pub enum FactoryOrderType {
    Suggested = 0,
    CombatRating = 1,
    CosmeticRating = 2,
    Added = 3,
    CPU = 4,
    MostBought = 5,
}

pub enum FactoryMovementType {
    Wheels = 100000,
    Hovers = 200000,
    Aerofoils=300000,
    Thrusters=400000,
    Rudders=500000,
    InsectLegs=600000,
    MechLegs=700000,
    Skis=800000,
    TankTreads=900000,
    Rotors=1000000,
    Sprinters=1100000,
    Propellers=1200000
}

pub enum FactoryWeaponType {
    Laser=10000000,
    PlasmaLauncher=20000000,
    GyroMortar=25000000,
    RailCannon=30000000,
    NanoDisruptor=40000000,
    TeslaBlade=50000000,
    AeroflakCannon=60000000,
    IonCannon=65000000,
    ProtoSeeker=70100000,
    ChainShredder=75000000,
}

pub enum FactoryTextSearchType {
    All=0,
    Player=1,
    Name=2,
}

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
    
    pub fn page(mut self, page_number: isize) -> Self {
        self.payload.page = page_number;
        self
    }
    
    pub fn items_per_page(mut self, page_size: isize) -> Self {
        self.payload.page_size = page_size;
        self
    }
    
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
    
    pub fn movement_or(mut self, movement_type: FactoryMovementType) -> Self {
        if self.payload.movement_filter == "" {
            self.payload.movement_filter = format!("{},{}", &self.payload.movement_filter, movement_type as isize);
        } else {
            self.payload.movement_filter = (movement_type as isize).to_string();
        }
        self.payload.movement_category_filter = self.payload.movement_filter.clone();
        self
    }
    
    pub fn weapon_or(mut self, weapon_type: FactoryWeaponType) -> Self {
        if self.payload.weapon_filter == "" {
            self.payload.weapon_filter = format!("{},{}", &self.payload.weapon_filter, weapon_type as isize);
        } else {
            self.payload.weapon_filter = (weapon_type as isize).to_string();
        }
        self.payload.weapon_category_filter = self.payload.weapon_filter.clone();
        self
    }
    
    pub fn cpu_range(mut self, min: isize, max: isize) -> Self {
        self.payload.minimum_cpu = min;
        self.payload.maximum_cpu = max;
        self
    }
    
    pub fn min_cpu(mut self, min: isize) -> Self {
        self.payload.minimum_cpu = min;
        self
    }
    
    pub fn max_cpu(mut self, max: isize) -> Self {
        self.payload.maximum_cpu = max;
        self
    }
    
    pub fn no_minimum_cpu(mut self) -> Self {
        self.payload.minimum_cpu = -1;
        self
    }
    
    pub fn no_maximum_cpu(mut self) -> Self {
        self.payload.maximum_cpu = -1;
        self
    }
    
    pub fn text(mut self, t: String) -> Self {
        self.payload.text_filter = t;
        self
    }
    
    pub fn text_search_type(mut self, search_type: FactoryTextSearchType) -> Self {
        self.payload.text_search_field = search_type as isize;
        self
    }
    
    // setting buyable to false while using the default token provider will cause HTTP status 500 error
    pub fn buyable(mut self, b: bool) -> Self {
        self.payload.buyable = b;
        self
    }
    
    pub fn prepend_featured(mut self, b: bool) -> Self {
        self.payload.prepend_featured_robot = b;
        self
    }
    
    pub fn default_page(mut self, b: bool) -> Self {
        self.payload.default_page = b;
        self
    }
    
    pub async fn send(mut self) -> Result<FactoryInfo, Error> {
        self.reqwest_builder = self.reqwest_builder.json(&self.payload);
        if let Some(token) = self.token.clone() {
            self.reqwest_builder = self.reqwest_builder.header("Authorization", "Web ".to_owned() + &token);
        }
        let result = self.reqwest_builder.send().await;
        if let Ok(response) = result {
            return response.json::<FactoryInfo>().await;
        }
        Err(result.err().unwrap())
    }
}

use ureq::{Request, Response, Error};

use crate::robocraft::{FactoryInfo, RoboShopItemsInfo, FactoryTextSearchType, FactoryWeaponType, FactoryMovementType, FactoryOrderType};
use crate::robocraft::{ListPayload};

#[derive(Clone)]
pub struct FactorySearchBuilder {
    reqwest_builder: Request,
    payload: ListPayload,
    token: Option<String>,
}

impl FactorySearchBuilder {
    pub(crate) fn new(request_builder: Request, token: Option<String>) -> FactorySearchBuilder {
        FactorySearchBuilder {
            reqwest_builder: request_builder.set("Content-Type", "application/json"),
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
    
    pub fn movement_raw(mut self, filter: String) -> Self {
        self.payload.movement_filter = filter.clone();
        self.payload.movement_category_filter = filter.clone();
        self
    }
    
    pub fn movement_or(mut self, movement_type: FactoryMovementType) -> Self {
        if self.payload.movement_filter == "" {
            self.payload.movement_filter = format!("{},{}", &self.payload.movement_filter, movement_type as isize);
        } else {
            self.payload.movement_filter = (movement_type as isize).to_string();
        }
        self.payload.movement_category_filter = self.payload.movement_filter.clone();
        self
    }
    
    pub fn weapon_raw(mut self, filter: String) -> Self {
        self.payload.weapon_filter = filter.clone();
        self.payload.weapon_category_filter = filter.clone();
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
    
    pub fn send(mut self) -> Result<FactoryInfo<RoboShopItemsInfo>, Error> {
        self.reqwest_builder = self.reqwest_builder;
        if let Some(token) = self.token.clone() {
            self.reqwest_builder = self.reqwest_builder.set("Authorization", &("Web ".to_owned() + &token));
        }
        let result = self.reqwest_builder.send_string(&serde_json::to_string(&self.payload).unwrap());
        if let Ok(response) = result {
            let json_res = response.into_json::<FactoryInfo<RoboShopItemsInfo>>();
            if let Ok(json) = json_res {
                return Ok(json);
            }
            return Err(Error::Status(500, Response::new(500, "Malformed JSON", "").unwrap())); // server returned malformed data
        }
        Err(result.err().unwrap())
    }
}

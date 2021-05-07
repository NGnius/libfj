use ureq::{Request, Response, Error};

use crate::robocraft::{FactoryInfo, RoboShopItemsInfo, FactoryTextSearchType, FactoryWeaponType, FactoryMovementType, FactoryOrderType};
use crate::robocraft::{ListPayload};

/// Factory API list query builder.
/// This is the simpler, blocking equivalent of libfj::robocraft::FactorySearchBuilder.
/// Please refer to that struct's documentation for details.
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
    
    /// Set page number
    pub fn page(mut self, page_number: isize) -> Self {
        self.payload.page = page_number;
        self
    }
    
    /// Set page size
    pub fn items_per_page(mut self, page_size: isize) -> Self {
        self.payload.page_size = page_size;
        self
    }
    
    /// Set results ordering
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
    
    /// Override movement filter
    pub fn movement_raw(mut self, filter: String) -> Self {
        self.payload.movement_filter = filter.clone();
        self.payload.movement_category_filter = filter.clone();
        self
    }
    
    /// Add allowed movement type
    pub fn movement_or(mut self, movement_type: FactoryMovementType) -> Self {
        if self.payload.movement_filter == "" {
            self.payload.movement_filter = format!("{},{}", &self.payload.movement_filter, movement_type as isize);
        } else {
            self.payload.movement_filter = (movement_type as isize).to_string();
        }
        self.payload.movement_category_filter = self.payload.movement_filter.clone();
        self
    }
    
    /// Override weapon filter
    pub fn weapon_raw(mut self, filter: String) -> Self {
        self.payload.weapon_filter = filter.clone();
        self.payload.weapon_category_filter = filter.clone();
        self
    }
    
    /// Add allowed weapon type
    pub fn weapon_or(mut self, weapon_type: FactoryWeaponType) -> Self {
        if self.payload.weapon_filter == "" {
            self.payload.weapon_filter = format!("{},{}", &self.payload.weapon_filter, weapon_type as isize);
        } else {
            self.payload.weapon_filter = (weapon_type as isize).to_string();
        }
        self.payload.weapon_category_filter = self.payload.weapon_filter.clone();
        self
    }
    
    /// Set CPU value min and max
    pub fn cpu_range(mut self, min: isize, max: isize) -> Self {
        self.payload.minimum_cpu = min;
        self.payload.maximum_cpu = max;
        self
    }
    
    /// Set CPU minimum value
    pub fn min_cpu(mut self, min: isize) -> Self {
        self.payload.minimum_cpu = min;
        self
    }
    
    /// Set CPU maximum value
    pub fn max_cpu(mut self, max: isize) -> Self {
        self.payload.maximum_cpu = max;
        self
    }
    
    /// Removem minimum CPU limit
    pub fn no_minimum_cpu(mut self) -> Self {
        self.payload.minimum_cpu = -1;
        self
    }
    
    /// Remove maximum CPU limit
    pub fn no_maximum_cpu(mut self) -> Self {
        self.payload.maximum_cpu = -1;
        self
    }
    
    /// Set text filter
    pub fn text(mut self, t: String) -> Self {
        self.payload.text_filter = t;
        self
    }
    
    /// Set fields which text filter searches
    pub fn text_search_type(mut self, search_type: FactoryTextSearchType) -> Self {
        self.payload.text_search_field = search_type as isize;
        self
    }
    
    // setting buyable to false while using the default token provider will cause HTTP status 500 error
    /// Only search robots which can be bought by the current account?
    pub fn buyable(mut self, b: bool) -> Self {
        self.payload.buyable = b;
        self
    }
    
    /// Prepend a featured robot to the response?
    pub fn prepend_featured(mut self, b: bool) -> Self {
        self.payload.prepend_featured_robot = b;
        self
    }
    
    /// Retrieve default CRF page?
    pub fn default_page(mut self, b: bool) -> Self {
        self.payload.default_page = b;
        self
    }
    
    /// Execute list query
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

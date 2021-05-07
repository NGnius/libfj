use ureq::{Agent, Error, Response};
use url::Url;
use serde_json::{to_string};

use crate::robocraft::{ITokenProvider, DefaultTokenProvider, FACTORY_DOMAIN, FactoryInfo, RoboShopItemsInfo, FactoryRobotGetInfo};
use crate::robocraft::{ListPayload};
use crate::robocraft_simple::FactorySearchBuilder;

/// Simpler CRF API implementation.
/// Refer to libfj::robocraft::FactoryAPI for in-depth documentation.
/// The only API difference is that this API is blocking (i.e. no async).
/// This version also works with Wine and Proton since it does not rely on tokio.
pub struct FactoryAPI {
    client: Agent,
    token: Box<dyn ITokenProvider>,
}

impl FactoryAPI {
    /// Create a new instance using `DefaultTokenProvider`.
    pub fn new() -> FactoryAPI {
        FactoryAPI {
            client: Agent::new(),
            token: Box::new(DefaultTokenProvider{}),
        }
    }
    
    /// List CRF robots
    pub fn list(&self) -> Result<FactoryInfo<RoboShopItemsInfo>, Error> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/api/roboShopItems/list")
            .unwrap();
        let payload = ListPayload::default();
        let mut request_builder = self.client.post(url.as_str())
            .set("Content-Type", "application/json");
        if let Ok(token) = self.token.token() {
            request_builder = request_builder.set("Authorization", &("Web ".to_owned() + &token));
        }
        let result = request_builder.send_string(&to_string(&payload).unwrap());
        if let Ok(response) = result {
            let json_res = response.into_json::<FactoryInfo<RoboShopItemsInfo>>();
            if let Ok(json) = json_res {
                return Ok(json);
            }
            return Err(Error::Status(500, Response::new(500, "Malformed JSON", "").unwrap())); // server returned malformed data
        }
        Err(result.err().unwrap())
    }
    
    /// Build a list query
    pub fn list_builder(&self) -> FactorySearchBuilder {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/api/roboShopItems/list")
            .unwrap();
        let mut token_opt = None;
        if let Ok(token) = self.token.token() {
            token_opt = Some(token);
        }
        let request_builder = self.client.post(url.as_str());
        FactorySearchBuilder::new(request_builder, token_opt)
    }
    
    /// Get complete information on a robot.
    pub fn get(&self, item_id: usize) -> Result<FactoryInfo<FactoryRobotGetInfo>, Error> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/api/roboShopItems/get/{}", item_id))
            .unwrap();
        let mut request_builder = self.client.get(url.as_str());
        if let Ok(token) = self.token.token() {
            request_builder = request_builder.set("Authorization", &("Web ".to_owned() + &token));
        }
        let result = request_builder.call();
        if let Ok(response) = result {
            let json_res = response.into_json::<FactoryInfo<FactoryRobotGetInfo>>();
            if let Ok(json) = json_res {
                return Ok(json);
            }
            return Err(Error::Status(500, Response::new(500, "Malformed JSON", "").unwrap())); // server returned malformed data
        }
        Err(result.err().unwrap())
    }
}

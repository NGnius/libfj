use reqwest::{Client, Error};
use url::{Url};

use crate::robocraft::{ITokenProvider, DefaultTokenProvider, FactoryInfo, FactorySearchBuilder, RoboShopItemsInfo, FactoryRobotGetInfo};
use crate::robocraft::factory_json::ListPayload;

const FACTORY_DOMAIN: &str = "https://factory.robocraftgame.com/";

pub struct FactoryAPI {
    client: Client,
    token: Box<dyn ITokenProvider>,
}

impl FactoryAPI {
    pub fn new() -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: Box::new(DefaultTokenProvider{}),
        }
    }
    
    pub fn with_auth(token_provider: Box<dyn ITokenProvider>) -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: token_provider,
        }
    }
    
    pub async fn list(&self) -> Result<FactoryInfo<RoboShopItemsInfo>, Error> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/api/roboShopItems/list")
            .unwrap();
        let payload = ListPayload::default();
        let mut request_builder = self.client.post(url)
            .json(&payload);
        if let Ok(token) = self.token.token() {
            request_builder = request_builder.header("Authorization", "Web ".to_owned() + &token);
        }
        let result = request_builder.send().await;
        if let Ok(response) = result {
            return response.json::<FactoryInfo<RoboShopItemsInfo>>().await;
        }
        Err(result.err().unwrap())
    }
    
    pub fn list_builder(&self) -> FactorySearchBuilder {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/api/roboShopItems/list")
            .unwrap();
        let mut token_opt = None;
        if let Ok(token) = self.token.token() {
            token_opt = Some(token);
        }
        let request_builder = self.client.post(url);
        FactorySearchBuilder::new(request_builder, token_opt)
    }
    
    pub async fn get(&self, item_id: usize) -> Result<FactoryInfo<FactoryRobotGetInfo>, Error> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/api/roboShopItems/get/{}", item_id))
            .unwrap();
        let mut request_builder = self.client.get(url);
        if let Ok(token) = self.token.token() {
            request_builder = request_builder.header("Authorization", "Web ".to_owned() + &token);
        }
        let result = request_builder.send().await;
        if let Ok(response) = result {
            return response.json::<FactoryInfo<FactoryRobotGetInfo>>().await;
        }
        Err(result.err().unwrap())
    }
}

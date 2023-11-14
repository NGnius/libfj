use std::sync::Mutex;

use reqwest::{Client, Error as ReqwestError, Response};
use url::{Url};

use crate::robocraft2::{ITokenProvider, ErrorPayload};
use crate::robocraft2::{SearchPayload, SearchResponse, CreateRobotPayload, CreateRobotResponse, FactoryInfoResponse, PublishRobotPayload, PublishRobotResponse, MyRobotsResponse, GetRobotResponse, ModerateRobotPayload, ReportRobotPayload};

/// Community Factory Robot 2 root URL
pub const FACTORY_DOMAIN: &str = "https://factory.production.robocraft2.com";

#[derive(Debug)]
pub enum FactoryError {
    Protocol(ReqwestError),
    Response(ErrorPayload),
    ResponseCode(ReqwestError, u16)
}

impl std::fmt::Display for FactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Protocol(p) => if let Some(status) = p.status() {
                write!(f, "HTTP Error {}: {}", status, p)
            } else {
                write!(f, "HTTP Error: {}", p)
            }
            Self::Response(r) => write!(f, "Factory Error #{}: {}", r.error, r.error_message),
            Self::ResponseCode(p, s) => write!(f, "HTTP Error {}: {}", s, p)
        }
    }
}

impl std::error::Error for FactoryError {}

async fn handle_json_response<D: for<'a> serde::Deserialize<'a>>(response: Response) -> Result<D, FactoryError> {
    let status_code: u16 = response.status().into();
    if status_code > 199 && status_code < 300 {
        Ok(response.json::<D>().await.map_err(FactoryError::Protocol)?)
    } else {
        match response.json::<ErrorPayload>().await {
            Ok(err) => Err(FactoryError::Response(err)),
            Err(e) => Err(FactoryError::ResponseCode(e, status_code))
        }
    }
}

/// CRF API implementation
pub struct FactoryAPI {
    client: Client,
    token: Mutex<Box<dyn ITokenProvider + Send>>,
}

impl FactoryAPI {
    /*/// Create a new instance, using `DefaultTokenProvider`.
    pub fn new() -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: Box::new(DefaultTokenProvider{}),
        }
    }*/

    /// Create a new instance using the provided token provider.
    pub fn with_auth(token_provider: Box<dyn ITokenProvider + Send>) -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: Mutex::new(token_provider),
        }
    }

    /// Retrieve CRF robots on the main page.
    pub async fn list(&self) -> Result<SearchResponse, FactoryError> {
        self.search(SearchPayload::default()).await
    }

    /// Search for robots on the CRF which meet the provided parameters
    pub async fn search(&self, params: SearchPayload) -> Result<SearchResponse, FactoryError> {
        let mut url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/v1/foundry/search")
            .unwrap();
        if let Some(text) = &params.text {
            url.query_pairs_mut().append_pair("text", text);
        }
        if let Some(base_minimum_cpu) = params.base_minimum_cpu {
            url.query_pairs_mut().append_pair("baseCpuMinimum", &base_minimum_cpu.to_string());
        }
        if let Some(base_maximum_cpu) = &params.base_maximum_cpu {
            url.query_pairs_mut().append_pair("baseCpuMaximum", &base_maximum_cpu.to_string());
        }
        if let Some(x) = &params.weapon_minimum_cpu {
            url.query_pairs_mut().append_pair("weaponCpuMinimum", &x.to_string());
        }
        if let Some(x) = &params.weapon_maximum_cpu {
            url.query_pairs_mut().append_pair("weaponCpuMaximum", &x.to_string());
        }
        if let Some(x) = &params.cosmetic_minimum_cpu {
            url.query_pairs_mut().append_pair("cosmeticCpuMinimum", &x.to_string());
        }
        if let Some(x) = &params.cosmetic_maximum_cpu {
            url.query_pairs_mut().append_pair("cosmeticCpuMaximum", &x.to_string());
        }
        if let Some(x) = &params.cluster_minimum {
            url.query_pairs_mut().append_pair("clusterMinimum", &x.to_string());
        }
        if let Some(x) = &params.cluster_maximum {
            url.query_pairs_mut().append_pair("clusterMaximum", &x.to_string());
        }
        if let Some(x) = &params.date_minimum {
            url.query_pairs_mut().append_pair("dateMinimum", x);
        }
        if let Some(x) = &params.date_maximum {
            url.query_pairs_mut().append_pair("dateMaximum", x);
        }
        if let Some(x) = &params.creator_id {
            url.query_pairs_mut().append_pair("creatorId", x);
        }
        if let Some(x) = &params.page {
            url.query_pairs_mut().append_pair("page", &x.to_string());
        }
        if let Some(x) = &params.count {
            url.query_pairs_mut().append_pair("count", &x.to_string());
        }
        url.query_pairs_mut().append_pair("sortBy", &params.sort_by);
        url.query_pairs_mut().append_pair("orderBy", &params.order_by);
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.get(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response::<SearchResponse>(result).await
    }

    pub async fn create_robot(&self, robot: CreateRobotPayload) -> Result<CreateRobotResponse, FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/v1/foundry/garage")
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.post(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .header("Content-Type", "application/json")
            .json(&robot)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response::<CreateRobotResponse>(result).await
    }

    pub async fn publish_robot(&self, robot: PublishRobotPayload, id: String) -> Result<PublishRobotResponse, FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/v1/foundry/vehicles/{}/publish", id))
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.post(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .header("Content-Type", "application/json")
            .json(&robot)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response(result).await
    }

    pub async fn unpublish_bot(&self, id: String) -> Result<(), FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/v1/foundry/vehicles/{}/unpublish", id))
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.post(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send().await
            .map_err(FactoryError::Protocol)?;
        let status_code = result.status().as_u16();
        if status_code > 199 && status_code < 300 {
            Ok(())
        } else {
            match result.json::<ErrorPayload>().await {
                Ok(err) => Err(FactoryError::Response(err)),
                Err(e) => Err(FactoryError::ResponseCode(e, status_code))
            }
        }
    }

    pub async fn delete_robot(&self, id: String) -> Result<(), FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/v1/foundry/vehicles/{}", id))
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.delete(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .header("Content-Type", "application/json")
            .send().await
            .map_err(FactoryError::Protocol)?;
        let status_code = result.status().as_u16();
        if status_code > 199 && status_code < 300 {
            Ok(())
        } else {
            match result.json::<ErrorPayload>().await {
                Ok(err) => Err(FactoryError::Response(err)),
                Err(e) => Err(FactoryError::ResponseCode(e, status_code))
            }
        }
    }

    pub async fn factory_info(&self) -> Result<FactoryInfoResponse, FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/v1/foundry/info")
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.get(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response::<FactoryInfoResponse>(result).await
    }

    pub async fn my_robots(&self) -> Result<MyRobotsResponse, FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/v1/foundry/garage")
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.get(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response::<MyRobotsResponse>(result).await
    }

    pub async fn my_published_robots(&self) -> Result<MyRobotsResponse, FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/v1/foundry/published")
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.get(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response::<MyRobotsResponse>(result).await
    }

    pub async fn get(&self, id: String) -> Result<GetRobotResponse, FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/v1/foundry/vehicles/{}", id))
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let result = self.client.get(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send().await
            .map_err(FactoryError::Protocol)?;
        handle_json_response::<GetRobotResponse>(result).await
    }

    pub async fn moderate(&self, payload: ModerateRobotPayload, id: String) -> Result<(), FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/v1/foundry/vehicles/{}/moderate", id))
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let _result = self.client.post(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await
            .map_err(FactoryError::Protocol)?;
        Ok(())
    }

    pub async fn report(&self, payload: ReportRobotPayload, id: String) -> Result<(), FactoryError> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join(&format!("/v1/foundry/vehicles/{}/report", id))
            .unwrap();
        let token = self.token.lock().unwrap().token().await.map_err(FactoryError::Protocol)?;
        let _result = self.client.post(url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await
            .map_err(FactoryError::Protocol)?;
        Ok(())
    }
}

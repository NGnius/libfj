use std::sync::Mutex;

use reqwest::{Client, Error};
use url::{Url};

use crate::robocraft2::{SearchPayload, SearchResponse, ITokenProvider};

/// Community Factory Robot 2 root URL
pub const FACTORY_DOMAIN: &str = "https://factory.production.robocraft2.com";

/// CRF API implementation
pub struct FactoryAPI {
    client: Client,
    token: Mutex<Box<dyn ITokenProvider>>,
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
    pub fn with_auth(token_provider: Box<dyn ITokenProvider>) -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: Mutex::new(token_provider),
        }
    }

    /// Retrieve CRF robots on the main page.
    ///
    /// For searching, use `list_builder()` instead.
    pub async fn list(&self) -> Result<SearchResponse, Error> {
        self.search(SearchPayload::default()).await
    }

    pub async fn search(&self, params: SearchPayload) -> Result<SearchResponse, Error> {
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
        let mut request_builder = self.client.get(url);
        if let Ok(token) = self.token.lock().unwrap().token().await {
            request_builder = request_builder.header("Authorization", "Bearer ".to_owned() + &token);
        }
        let result = request_builder.send().await?;
        result.json::<SearchResponse>().await
    }
}

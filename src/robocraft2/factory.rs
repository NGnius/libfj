use reqwest::{Client, Error};
use url::{Url};

use crate::robocraft::{ITokenProvider, DefaultTokenProvider};
use crate::robocraft2::{SearchPayload, SearchResponse};

/// Community Factory Robot 2 root URL
pub const FACTORY_DOMAIN: &str = "https://factory.production.robocraft2.com";

/// CRF API implementation
pub struct FactoryAPI {
    client: Client,
    token: Box<dyn ITokenProvider>,
}

impl FactoryAPI {
    /// Create a new instance, using `DefaultTokenProvider`.
    pub fn new() -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: Box::new(DefaultTokenProvider{}),
        }
    }

    /// Create a new instance using the provided token provider.
    pub fn with_auth(token_provider: Box<dyn ITokenProvider>) -> FactoryAPI {
        FactoryAPI {
            client: Client::new(),
            token: token_provider,
        }
    }

    /// Retrieve CRF robots on the main page.
    ///
    /// For searching, use `list_builder()` instead.
    pub async fn list(&self) -> Result<SearchResponse, Error> {
        let url = Url::parse(FACTORY_DOMAIN)
            .unwrap()
            .join("/v1/foundry/search")
            .unwrap();
        let mut request_builder = self.client.get(url);
        if let Ok(token) = self.token.token() {
            request_builder = request_builder.header("Authorization", "Bearer ".to_owned() + &token);
        }
        let result = request_builder.send().await?;
        result.json::<SearchResponse>().await
    }

    async fn search(&self, params: SearchPayload) -> Result<SearchResponse, Error> {
        todo!()
    }
}

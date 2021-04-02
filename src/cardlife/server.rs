use reqwest::{Client, IntoUrl, Error};
use url::{Origin, Url};
use crate::cardlife::{GameInfo, StatusInfo};

pub struct CLreServer {
    client: Client,
    addr: Url,
}

impl CLreServer {
    pub fn new<U: IntoUrl>(url: U) -> Result<CLreServer, ()> {
        let url_result = url.into_url();
        if let Ok(uri) = url_result {
            if let Origin::Tuple(scheme, host, port) = uri.origin() {
                if let Ok(addr) = Url::parse(&format!("{}://{}:{}", scheme, host.to_string(), port)) {
                    return Ok(
                        CLreServer {
                            client: Client::new(),
                            addr,
                        }
                    );
                }
            }
        }
        Err(())
    }

    pub async fn game_info(&self) -> Result<GameInfo, Error> {
        let response = self.client.get(self.addr.join("/c/game.json").unwrap())
            .send().await;
        if let Ok(resp) = response {
            return resp.json::<GameInfo>().await
        }
        Err(response.err().unwrap())
    }

    pub async fn status_info(&self) -> Result<StatusInfo, Error> {
        let response = self.client.get(self.addr.join("/status.json").unwrap())
            .send().await;
        if let Ok(resp) = response {
            return resp.json::<StatusInfo>().await
        }
        Err(response.err().unwrap())
    }
}


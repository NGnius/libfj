use reqwest::{Client, Error};
use url::{Url};

use crate::cardlife::{AuthenticationInfo, AuthenticationPayload, LobbyInfo, LobbyPayload};

const AUTHENTICATION_DOMAIN: &str = "https://live-auth.cardlifegame.com/";
const LOBBY_DOMAIN: &str = "https://live-lobby.cardlifegame.com/";

/// Cardlife live information API
pub struct LiveAPI {
    client: Client,
    auth: Option<AuthenticationInfo>,
}

impl LiveAPI {
    /// Create a new instance
    pub fn new() -> LiveAPI {
        LiveAPI {
            client: Client::new(),
            auth: None,
        }
    }

    /// Create a new instance and login using email
    pub async fn login_email(email: &str, password: &str) -> Result<LiveAPI, Error> {
        let mut instance = LiveAPI::new();
        let result = instance.authenticate_email(email, password).await;
        if let Ok(response) = result {
            instance.auth = Some(response);
            return Ok(instance);
        } else {
            return Err(result.err().unwrap());
        }
    }
    
    /// Login using email and password
    pub async fn authenticate_email(&mut self, email: &str, password: &str) -> Result<AuthenticationInfo, Error> {
        let url = Url::parse(AUTHENTICATION_DOMAIN)
            .unwrap()
            .join("/api/auth/authenticate")
            .unwrap();
        let payload = AuthenticationPayload {
            email_address: email.to_string(),
            password: password.to_string()
        };
        let result = self.client.post(url)
            .json(&payload)
            .send().await;
        if let Ok(response) = result {
            let res = response.json::<AuthenticationInfo>().await;
            if let Ok(auth) = &res {
                self.auth = Some(auth.clone());
            }
            return res;
        }
        Err(result.err().unwrap())
    }
    
    // TODO username authentication

    /// Retrieve lobby information for all active Cardlife servers
    pub async fn lobbies(&self) -> Result<LobbyInfo, Error> {
        let url = Url::parse(LOBBY_DOMAIN)
            .unwrap()
            .join("/api/client/games")
            .unwrap();
        let public_id;
        if let Some(auth) = &self.auth {
            public_id = auth.public_id.clone();
        } else {
            public_id = "".to_string();
        }
        let payload = LobbyPayload{public_id};
        let result = self.client.post(url).json(&payload).send().await;
        if let Ok(response) = result {
            return response.json::<LobbyInfo>().await;
        }
        Err(result.err().unwrap())
    }
}

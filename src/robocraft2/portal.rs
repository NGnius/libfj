use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use ureq::{Agent, Error};
use serde_json::to_string;

use crate::robocraft::{ITokenProvider, account::AuthenticationResponseInfo, AccountInfo};

/// Token provider for an existing Freejam account, authenticated through the web browser portal.
///
/// Steam and Epic accounts are not supported.
pub struct PortalTokenProvider {
    /// Login token
    token: RwLock<ProgressionLoginResponse>,
    /// User info token
    jwt: AuthenticationResponseInfo,
    /// Ureq HTTP client
    client: Agent,
}

impl PortalTokenProvider {
    pub fn portal() -> Result<Self, Error> {
        Self::target("Techblox".to_owned())
    }

    pub fn target(value: String) -> Result<Self, Error> {
        let client = Agent::new();
        let payload = PortalStartPayload {
            target: value,
        };
        let start_response = client.post("https://account.freejamgames.com/api/authenticate/portal/start")
            .set("Content-Type", "application/json")
            .send_string(&to_string(&payload).unwrap())?;
        let start_res = start_response.into_json::<PortalStartResponse>()?;

        println!("GO TO https://account.freejamgames.com/login?theme=rc2&redirect_url=portal?theme=rc2%26portalToken={}", start_res.token);

        let payload = PortalCheckPayload {
            token: start_res.token,
        };
        let mut check_response = client.post("https://account.freejamgames.com/api/authenticate/portal/check")
            .set("Content-Type", "application/json")
            .send_json(&payload)?;
            //.send_string(&to_string(&payload).unwrap())?;
        let mut auth_complete = check_response.status() == 200;
        while !auth_complete {
            check_response = client.post("https://account.freejamgames.com/api/authenticate/portal/check")
                .set("Content-Type", "application/json")
                .send_json(&payload)?;
            auth_complete = check_response.status() == 200;
        }
        let check_res = check_response.into_json::<AuthenticationResponseInfo>()?;

        // login with token we just got
        let payload = ProgressionLoginPayload {
            token: check_res.token.clone(),
        };
        let progress_response = client.post("https://progression.production.robocraft2.com/login/fj")
            .set("Content-Type", "application/json")
            .send_json(&payload)?;
        let progress_res = progress_response.into_json::<ProgressionLoginResponse>()?;
        Ok(Self {
            token: RwLock::new(progress_res),
            jwt: check_res,
            client: client,
        })
    }

    pub fn get_account_info(&self) -> Result<AccountInfo, Error> {
        Ok(self.jwt.decode_jwt_data())
    }
}

impl ITokenProvider for PortalTokenProvider {
    fn token(&self) -> Result<String, ()> {
        // TODO re-authenticate when expired
        if let Some(token) = self.token.read().map_err(|_| ())?.token.clone() {
            Ok(token)
        } else {
            Err(())
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct PortalStartPayload {
    #[serde(rename = "Target")]
    pub target: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct PortalStartResponse {
    #[serde(rename = "Token")]
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct PortalCheckPayload {
    #[serde(rename = "Token")]
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct ProgressionLoginPayload {
    #[serde(rename = "token")]
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct ProgressionLoginResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[serde(rename = "serverToken")]
    pub server_token: Option<String>,
}

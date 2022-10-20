use serde::{Deserialize, Serialize};
//use ureq::{Agent, Error, AgentBuilder};
use reqwest::{Client, Error};
//use cookie_store::CookieStore;
//use url::{Url};
use serde_json::from_slice;
use chrono::{DateTime, naive::NaiveDateTime, Utc};

/// Token generator for authenticated API endpoints
#[async_trait::async_trait]
pub trait ITokenProvider {
    /// Retrieve the token to use
    async fn token(&mut self) -> Result<String, Error>;
}

/// Token provider for an existing Freejam account, authenticated through the web browser portal.
///
/// Steam and Epic accounts are not supported.
pub struct PortalTokenProvider {
    /// Login token
    token: ProgressionLoginResponse,
    /// User info token
    jwt: PortalCheckResponse,
    /// Ureq HTTP client
    client: Client,
    /// target game
    target: String,
}

impl PortalTokenProvider {
    /// Login through the web browser portal
    pub async fn portal() -> Result<Self, Error> {
        Self::target("Techblox".to_owned()).await
    }

    /// Login through the portal with a custom target value
    pub async fn target(value: String) -> Result<Self, Error> {
        let client = Client::new();
        let payload = PortalStartPayload {
            target: value.clone(),
        };
        let start_response = client.post("https://account.freejamgames.com/api/authenticate/portal/start")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        let start_res = start_response.json::<PortalStartResponse>().await?;

        println!("GO TO https://account.freejamgames.com/login?theme=rc2&redirect_url=portal?theme=rc2%26portalToken={}", start_res.token);

        let payload = PortalCheckPayload {
            token: start_res.token,
        };
        let mut check_response = client.post("https://account.freejamgames.com/api/authenticate/portal/check")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        let mut auth_complete = check_response.status() == 200;
        while !auth_complete {
            check_response = client.post("https://account.freejamgames.com/api/authenticate/portal/check")
                .header("Content-Type", "application/json")
                .json(&payload)
                .send().await?;
            auth_complete = check_response.status() == 200;
        }
        let check_res = check_response.json::<PortalCheckResponse>().await?;

        // login with token we just got
       Self::login_internal(check_res, client, value).await
    }

    pub async fn with_email(email: &str, password: &str) -> Result<Self, Error> {
        let client = Client::new();
        let payload = AuthenticationEmailPayload {
            email_address: email.to_string(),
            password: password.to_string(),
        };
        let response = client.post("https://account.freejamgames.com/api/authenticate/email/web")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        let json_res = response.json::<AuthenticationResponseInfo>().await?;
        Self::auto_portal(client, "Techblox".to_owned(), json_res.token).await
    }

    pub async fn with_username(username: &str, password: &str) -> Result<Self, Error> {
        let client = Client::new();
        let payload = AuthenticationUsernamePayload {
            username: username.to_string(),
            password: password.to_string(),
        };
        let response = client.post("https://account.freejamgames.com/api/authenticate/displayname/web")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        let json_res = response.json::<AuthenticationResponseInfo>().await?;
        Self::auto_portal(client, "Techblox".to_owned(), json_res.token).await
    }

    /// Automatically validate portal
    async fn auto_portal(client: Client, value: String, token: String) -> Result<Self, Error> {
        let payload = PortalStartPayload {
            target: value.clone(),
        };
        let start_response = client.post("https://account.freejamgames.com/api/authenticate/portal/start")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        let start_res = start_response.json::<PortalStartResponse>().await?;

        let payload = PortalCheckPayload {
            token: start_res.token,
        };

        let _assign_response = client.post("https://account.freejamgames.com/api/authenticate/portal/assign")
            .header("Content-Type", "application/json")
            .header("Authorization", "Web ".to_owned() + &token)
            .json(&payload)
            .send().await?;

        let check_response = client.post("https://account.freejamgames.com/api/authenticate/portal/check")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        let check_res = check_response.json::<PortalCheckResponse>().await?;

        // login with token we just got
       Self::login_internal(check_res, client, value).await
    }

    async fn login_internal(token_data: PortalCheckResponse, client: Client, target: String) -> Result<Self, Error> {
        let progress_res = Self::login_step(&token_data, &client).await?;
        Ok(Self {
            token: progress_res,
            jwt: token_data,
            client: client,
            target: target,
        })
    }

    async fn login_step(token_data: &PortalCheckResponse, client: &Client) -> Result<ProgressionLoginResponse, Error> {
        let payload = ProgressionLoginPayload {
            token: token_data.token.clone(),
        };
        let progress_response = client.post("https://progression.production.robocraft2.com/login/fj")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send().await?;
        progress_response.json::<ProgressionLoginResponse>().await
    }

    /// Login using the portal token data from a previous portal authentication
    pub async fn login(token_data: PortalCheckResponse, target: String) -> Result<Self, Error> {
        Self::login_internal(token_data, Client::new(), target).await
    }

    pub fn get_account_info(&self) -> Result<AccountInfo, Error> {
        Ok(self.jwt.decode_jwt_data())
    }

    pub fn token_data(&self) -> &'_ PortalCheckResponse {
        &self.jwt
    }
}

#[async_trait::async_trait]
impl ITokenProvider for PortalTokenProvider {
    async fn token(&mut self) -> Result<String, Error> {
        let decoded_jwt = self.jwt.decode_jwt_data();
        let expiry = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(decoded_jwt.exp as i64, 0), Utc);
        let now = Utc::now();
        if now >= expiry || self.token.token.is_none() {
            // refresh token when expired
            // TODO make sure refresh token isn't also expired
            // (it would be a bit concerning if you decide to run libfj for 1+ month, though)
            let payload = RefreshTokenPayload {
                target: self.target.clone(),
                refresh_token: self.jwt.refresh_token.clone(),
                public_id: decoded_jwt.public_id,
            };
            let refresh_response = self.client.post("https://account.freejamgames.com/api/authenticate/token/refresh")
                .header("Content-Type", "application/json")
                .json(&payload)
                .send().await?;
            self.jwt = refresh_response.json::<PortalCheckResponse>().await?;
            self.token = Self::login_step(&self.jwt, &self.client).await?;
        }
        Ok(self.token.token.clone().unwrap())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct AuthenticationEmailPayload {
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct AuthenticationUsernamePayload {
    #[serde(rename = "DisplayName")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct AuthenticationResponseInfo {
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "RefreshTokenExpiry")]
    pub refresh_token_expiry: String,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PortalCheckResponse {
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "RefreshTokenExpiry")]
    pub refresh_token_expiry: String,
}

impl PortalCheckResponse {
    pub fn decode_jwt_data(&self) -> AccountInfo {
        // Refer to https://jwt.io/
        // header is before dot, signature is after dot.
        // data is sandwiched in the middle, and it's all we care about
        let data = self.token.split(".").collect::<Vec<&str>>()[1];
        let data_vec = base64::decode(data).unwrap();
        from_slice::<AccountInfo>(&data_vec).unwrap()
    }
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct RefreshTokenPayload {
    #[serde(rename = "Target")]
    pub target: String, // "Techblox"
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "PublicId")]
    pub public_id: String,
}

/// Robocraft2 account information.
#[derive(Deserialize, Serialize, Clone)]
pub struct AccountInfo {
    /// User's public ID
    #[serde(rename = "PublicId")]
    pub public_id: String,
    /// Account display name
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// Account GUID, or display name for older accounts
    #[serde(rename = "RobocraftName")]
    pub robocraft_name: String,
    /// ??? is confirmed?
    #[serde(rename = "Confirmed")]
    pub confirmed: bool,
    /// Freejam support code
    #[serde(rename = "SupportCode")]
    pub support_code: String,
    /// User's email address
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    /// Email address is verified?
    #[serde(rename = "EmailVerified")]
    pub email_verified: bool,
    /// Account creation date
    #[serde(rename = "CreatedDate")]
    pub created_date: String,
    /// Owned products (?)
    #[serde(rename = "Products")]
    pub products: Vec<String>,
    /// Account flags
    #[serde(rename = "Flags")]
    pub flags: Vec<String>,
    /// Account has a password?
    #[serde(rename = "HasPassword")]
    pub has_password: bool,
    /// Mailing lists that the account is signed up for
    #[serde(rename = "MailingLists")]
    pub mailing_lists: Vec<String>,
    /// Is Steam account? (always false)
    #[serde(rename = "HasSteam")]
    pub has_steam: bool,
    /// iss (?)
    #[serde(rename = "iss")]
    pub iss: String,
    /// sub (?)
    #[serde(rename = "sub")]
    pub sub: String,
    /// Token created at (unix time) (?)
    #[serde(rename = "iat")]
    pub iat: u64,
    /// Token expiry (unix time) (?)
    #[serde(rename = "exp")]
    pub exp: u64,
}

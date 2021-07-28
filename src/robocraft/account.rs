use serde::{Deserialize, Serialize};
use ureq::{Agent, Error};
use serde_json::{to_string, from_slice};

use crate::robocraft::ITokenProvider;

/// Token provider for an existing Robocraft account.
///
/// Steam accounts are not supported.
pub struct AuthenticatedTokenProvider {
    /// The account's username
    pub username: String,
    /// The account's password
    pub password: String,
    /// Ureq HTTP client
    client: Agent,
}

impl AuthenticatedTokenProvider {
    pub fn with_email(email: &str, password: &str) -> Result<Self, Error> {
        let client = Agent::new();
        let payload = AuthenticationEmailPayload {
            email_address: email.to_string(),
            password: password.to_string(),
        };
        let response = client.post("https://account.freejamgames.com/api/authenticate/email/web")
            .set("Content-Type", "application/json")
            .send_string(&to_string(&payload).unwrap())?;
        let json_res = response.into_json::<AuthenticationResponseInfo>()?;
        Ok(Self {
            username: json_res.decode_jwt_data().display_name,
            password: password.to_string(),
            client,
        })
    }

    pub fn with_username(username: &str, password: &str) -> Result<Self, Error> {
        let new_obj = Self {
            username: username.to_string(),
            password: password.to_string(),
            client: Agent::new(),
        };
        new_obj.do_auth()?;
        Ok(new_obj)
    }

    fn do_auth(&self) -> Result<AuthenticationResponseInfo, Error> {
        let payload = AuthenticationUsernamePayload {
            username: self.username.clone(),
            password: self.password.clone(),
        };
        let response = self.client.post("https://account.freejamgames.com/api/authenticate/displayname/web")
            .set("Content-Type", "application/json")
            .send_string(&to_string(&payload).unwrap())?;
        let json_res = response.into_json::<AuthenticationResponseInfo>()?;
        Ok(json_res)
    }

    pub fn get_account_info(&self) -> Result<AccountInfo, Error> {
        let json_res = self.do_auth()?;
        Ok(json_res.decode_jwt_data())
    }
}

impl ITokenProvider for AuthenticatedTokenProvider {
    fn token(&self) -> Result<String, ()> {
        let json_res = self.do_auth().map_err(|_|())?;
        Ok(json_res.token)
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

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct AuthenticationResponseInfo {
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "RefreshTokenExpiry")]
    pub refresh_token_expiry: String,
}

impl AuthenticationResponseInfo {
    pub fn decode_jwt_data(&self) -> AccountInfo {
        // Refer to https://jwt.io/
        // header is before dot, signature is after dot.
        // data is sandwiched in the middle, and it's all we care about
        let data = self.token.split(".").collect::<Vec<&str>>()[1];
        let data_vec = base64::decode(data).unwrap();
        from_slice::<AccountInfo>(&data_vec).unwrap()
    }
}

/// Robocraft account information.
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

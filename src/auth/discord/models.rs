use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TokenExchangeBody {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}
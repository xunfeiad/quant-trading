use crate::Signature;
use crate::error::{Error, Result};
use quant_config::{Credentials, ExchangeConfig, USER_CONFIG};
use quant_schema::Exchange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub op: String,
    pub args: LoginArgs,
    #[serde(skip)]
    pub is_testnet: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginArgs {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub passphrase: String,
    pub timestamp: Option<String>,
    pub sign: Option<String>,
}

impl Signature for LoginArgs {
    fn secret(&self) -> Result<String> {
        let credentials: Credentials = USER_CONFIG
            .get()
            .ok_or(Error::Other("USER CONFIG not initialized."))?
            .get_exchange(&Exchange::Okex)
            .ok_or_else(|| Error::ConfigError("Exchange::Okex not found in config"))?
            .into();
        Ok(credentials.secret_key.clone())
    }

    fn merged_string(
        &self,
        method: &reqwest::Method,
        path: &str,
        body: &str,
        timestamp: &str,
    ) -> Result<String> {
        let merged = format!("{}{}{}{}", timestamp, method.as_str(), path, body);
        Ok(merged)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub event: String,
    pub code: String,
    pub msg: String,
    #[serde(rename = "connId")]
    pub conn_id: String,
}

pub mod error;

pub use error::Error;
use quant_schema::Exchange;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use std::sync::OnceLock;

pub static USER_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Support multiple exchanges
    pub exchanges: Mutex<HashMap<Exchange, ExchangeConfig>>,

    /// Default exchange configuration
    pub default_exchange: Option<Exchange>,

    /// Global configuration settings
    #[serde(rename = "global")]
    pub global_config: GlobalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Default timeout
    pub timeout: u64,

    /// Default retry attempts
    pub retry_count: u32,

    /// Default logging level
    pub log_level: Option<String>,

    /// Default retry delay in milliseconds
    pub retry_delay: u64,

    /// Use testnet or mainnet
    pub use_testnet: bool,

    /// Use `websocket` or `http` for market data
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExchangeConfig {
    #[serde(rename = "okex")]
    Okex(OkexConfig),
    #[serde(rename = "binance")]
    Binance(BinanceConfig),
}

impl Into<Credentials> for ExchangeConfig {
    fn into(self) -> Credentials {
        match self {
            ExchangeConfig::Okex(okex_config) => okex_config.credentials,
            ExchangeConfig::Binance(binance_config) => binance_config.credentials,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkexConfig {
    pub credentials: Credentials,
    /// Base URL for Okex API
    pub http_urls: Vec<String>,
    pub ws_urls: Vec<String>,

    pub timeout: Option<u64>,
    pub retry_count: Option<u16>,
    pub retry_delay: Option<u64>,
        pub testnet_http_urls: Option<Vec<String>>,
    pub testnet_ws_urls: Option<Vec<String>>,
    pub ip_blacks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceConfig {
    pub credentials: Credentials,
    pub http_urls: Vec<String>,
    pub ws_urls: Vec<String>,
    pub timeout: Option<u64>,
    pub retry_count: Option<u16>,
    pub retry_delay: Option<u64>,
    pub testnet_http_urls: Option<Vec<String>>,
    pub testnet_ws_urls: Option<Vec<String>>,
    pub ip_blacks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials{
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: String,
    pub use_testnet: Option<bool>,
    pub protocol: Option<String>
}

impl Config {
    /// Get specific exchange configuration by name
    pub fn get_exchange(&self, exchange: &Exchange) -> Option<ExchangeConfig> {
        let exchanges = self.exchanges.lock().unwrap();
        exchanges.get(&exchange).cloned()
    }

    /// Get default exchange configuration
    pub fn get_default_exchange(&self) -> Option<ExchangeConfig> {
        if let Some(default_exchange) = &self.default_exchange {
            self.get_exchange(default_exchange)
        } else {
            self.get_exchange(&Exchange::default())
        }
    }

    /// Add or update an exchange configuration
    pub fn set_exchange(&self, exchange: Exchange, config: ExchangeConfig) {
        let mut exchanges = self.exchanges.lock().unwrap();
        exchanges.entry(exchange).or_insert(config);
    }

    /// Check if an exchange configuration exists
    pub fn has_exchange(&self, exchange: &Exchange) -> bool {
        let exchanges = self.exchanges.lock().unwrap();
        exchanges.contains_key(exchange)
    }

    /// Load configuration from a TOML file
    pub fn load_from_file(file_path: &str) -> Result<(), Error> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_str)?;
        USER_CONFIG
            .set(config)
            .map_err(|_| Error::ConfigError("Failed to set user config".to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    #[test]
    pub fn test_load_file() -> Result<(), Error> {
        Config::load_from_file("config.toml")?;
        let config = USER_CONFIG.get().unwrap();
        assert!(config.has_exchange(&Exchange::Okex));
        assert!(config.has_exchange(&Exchange::Binance));
        Ok(())
    }
}

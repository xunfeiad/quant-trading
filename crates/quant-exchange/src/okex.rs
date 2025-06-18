use crate::constant::okx_endpoints;
use crate::error::{Error, Result};
use crate::schema::okex::login::{LoginArgs, LoginRequest};
use crate::{ExchangeClient, ExchangeHttpClient, ExchangeWsClientTrait, Signature};
use async_trait::async_trait;
use bytes::Bytes;
use quant_config::{Config, Credentials, OkexConfig, USER_CONFIG};
use quant_schema::Exchange;
use reqwest::{Method, header::HeaderMap};
use tokio_tungstenite::tungstenite::handshake::client::Request;

pub struct OkexClient<'a> {
    pub http_client: &'a reqwest::Client,
    pub config: &'a Config,
    pub ws_is_connected: bool,
}

#[async_trait]
impl ExchangeClient for OkexClient<'_> {
    fn exchange_client_headers(&self) -> HeaderMap {
        let headers = HeaderMap::new();
        headers
    }

    fn base_url(&self) -> Result<String> {
        let credentials: Credentials = self.get_config()?;
        let Credentials {
            use_testnet,
            protocol,
            ..
        } = credentials;

        let use_testnet = use_testnet.unwrap_or(false);
        let protocol = protocol.as_deref().unwrap_or("https");

        match (use_testnet, protocol) {
            (true, "https") => Ok(okx_endpoints::DEMO_REST_BASE_URL.to_string()),
            (true, "wss") => Ok(okx_endpoints::DEMO_WS_PUBLIC_URL.to_string()),
            (false, "https") => Ok(okx_endpoints::PROD_REST_BASE_URL.to_string()),
            (false, "wss") => Ok(okx_endpoints::PROD_WS_PUBLIC_URL.to_string()),
            _ => Err(Error::ConfigError(
                "Invalid protocol or testnet configuration",
            )),
        }
    }

    fn sign_request(&self, method: &Method, path: &str, body: &str) -> Result<HeaderMap> {
        let config = self.get_config()?;

        let mut headers = self.exchange_client_headers();
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let sign = self.gen_signature(method, path, body, &timestamp)?;
        // Here you would implement the signing logic specific to Okex
        // For example, you might need to add an API key, signature, etc.
        // This is a placeholder implementation.
        headers.insert("OK-ACCESS-KEY", config.api_key.parse()?);
        headers.insert("OK-ACCESS-SIGN", sign.parse().unwrap());
        headers.insert("OK-ACCESS-TIMESTAMP", timestamp.parse()?);
        if config.use_testnet.unwrap_or(false) && config.protocol.as_deref() == Some("wss") {
            headers.insert("x-simulated-trading", "1".parse()?);
        }
        Ok(headers)
    }
}

#[async_trait]
impl ExchangeHttpClient for OkexClient<'_> {
    async fn login(&self) -> Result<()> {
        Ok(())
    }
    fn http_client(&self) -> &reqwest::Client {
        self.http_client
    }
}

impl OkexClient<'_> {
    pub fn get_config(&self) -> Result<Credentials> {
        let config = USER_CONFIG
            .get()
            .ok_or(Error::Other("USER CONFIG not initialized."))?
            .get_exchange(&Exchange::Okex)
            .ok_or_else(|| Error::ConfigError("Exchange::Okex not found in config"))?;
        Ok(config.into())
    }
}

impl Signature for OkexClient<'_> {
    fn secret(&self) -> Result<String> {
        let config = self.get_config()?;
        Ok(config.secret_key.clone())
    }

    fn merged_string(
        &self,
        method: &Method,
        path: &str,
        body: &str,
        timestamp: &str,
    ) -> Result<String> {
        let config = self.get_config()?;
        Ok(format!(
            "{}{}{}{}{}{}",
            &config.api_key,
            &config.secret_key,
            &timestamp,
            method.as_str(),
            path,
            body
        ))
    }
}

#[async_trait]
impl ExchangeWsClientTrait for OkexClient<'_> {
    fn login_request(&self) -> Result<String> {
        let config = self.get_config()?;
        let login_args = LoginArgs {
            api_key: config.api_key.clone(),
            passphrase: config.passphrase.clone(),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            sign: None, // Sign will be added later
        };

        let login_request = LoginRequest {
            op: "login".to_string(),
            args: login_args,
            is_testnet: config.use_testnet.unwrap_or(false),
        };

        let body = serde_json::to_string(&login_request)?;
        Ok(body)
    }

    fn is_connected(&self) -> bool {
        self.ws_is_connected
    }

    async fn close(&mut self) -> Result<()> {
        self.ws_is_connected = false;
        Ok(())
    }

    async fn reconnect(&mut self) -> Result<()> {
        self.ws_is_connected = true;
        Ok(())
    }

    fn set_connected_state(&mut self, state: bool) {
        self.ws_is_connected = state;
    }

    fn request(&self) -> Result<Request> {
        let headers = self.sign_request(&Method::GET, "/users/self/verify", "")?;
        println!("Headers: {:?}", headers);
        let mut request = Request::builder().uri(self.base_url()?).body(())?;
        let reuest_headers = request.headers_mut();
        for (key, value) in headers {
            if key.is_some() {
                reuest_headers.insert(key.unwrap(), value);
            }
        }
        Ok(request)
    }
}

#[cfg(test)]
mod test {
    use futures_util::SinkExt;
    use tokio_tungstenite::tungstenite::Message;

    use crate::schema::okex::channel::WsChannelType;

    use super::*;

    #[tokio::test]
    async fn test_ws_login() {
        Config::load_from_file("../../config.toml").expect("Failed to load config");
        let config = USER_CONFIG.get().unwrap();
        let credentials: Credentials = config.get_exchange(&Exchange::Okex).unwrap().into();
        println!("{:?}", credentials);
        let mut client = OkexClient {
            http_client: &reqwest::Client::new(),
            config: config,
            ws_is_connected: false,
        };
        let (mut ws_sink, ws_reader) = client
            .get_ws_stream(WsChannelType::Public)
            .await
            .expect("Failed to get WebSocket stream");

        let pong = ws_sink.send(Message::Ping(Bytes::from("value"))).await;
        println!("Pong response: {:?}", pong);
        assert!(pong.is_ok(), "Failed to send ping");
        assert!(false)
    }
}

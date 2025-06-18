pub mod binance;
pub mod constant;
pub mod error;
pub mod okex;
pub mod schema;

use crate::schema::okex::channel::WsChannelType;
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose};
use error::Result;
use futures_util::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use hmac::{Hmac, Mac};
use reqwest::{Client, Method, Response, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::protocol::Message};

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsReader = SplitStream<WsStream>;

#[async_trait]
pub trait ExchangeClient {
    fn exchange_client_headers(&self) -> HeaderMap;

    fn base_url(&self) -> Result<String>;

    fn sign_request(&self, method: &Method, path: &str, body: &str) -> Result<HeaderMap>;
}

#[async_trait]
pub trait ExchangeHttpClient: ExchangeClient {
    async fn login(&self) -> Result<()>;

    fn http_client(&self) -> &Client;
    async fn send_request<T: Serialize + Sync>(
        &self,
        method: &Method,
        path: &str,
        query_params: Option<&HashMap<String, String>>,
        body: Option<&T>,
        authenticated: bool,
    ) -> Result<Response> {
        let mut url = format!("{}{}", self.base_url()?, path);

        // 添加查询参数
        if let Some(params) = query_params {
            if !params.is_empty() {
                let query_string: Vec<String> =
                    params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
                url = format!("{}?{}", url, query_string.join("&"));
            }
        }

        let body_str = match body {
            Some(b) => serde_json::to_string(b)?,
            None => String::new(),
        };

        // 构建请求
        let mut request_builder = self.http_client().request(method.clone(), &url);

        // 添加基础头部
        let mut headers = self.exchange_client_headers();

        // 如果需要认证，添加签名头部
        if authenticated {
            let auth_headers = self.sign_request(&method, path, &body_str)?;
            headers.extend(auth_headers);
        }

        request_builder = request_builder.headers(headers);

        // 添加请求体
        if !body_str.is_empty() {
            request_builder = request_builder.body(body_str.to_string());
        }

        // 发送请求
        let response = request_builder.send().await?;

        Ok(response)
    }
}

pub trait Signature {
    fn secret(&self) -> Result<String>;
    fn merged_string(
        &self,
        method: &Method,
        path: &str,
        body: &str,
        timestamp: &str,
    ) -> Result<String>;
    fn gen_signature(
        &self,
        method: &Method,
        path: &str,
        body: &str,
        timestamp: &str,
    ) -> Result<String> {
        let mut mac = Hmac::<sha2::Sha256>::new_from_slice(self.secret()?.as_bytes())?;
        mac.update(
            self.merged_string(method, path, body, timestamp)?
                .as_bytes(),
        );
        let result = mac.finalize();
        let signature = general_purpose::STANDARD.encode(result.into_bytes());
        Ok(signature)
    }
}

#[async_trait]
pub trait ExchangeWsClientTrait: ExchangeClient {
    fn request(&self) -> Result<Request>;

    fn login_request(&self) -> Result<String>;
    async fn get_ws_stream(&mut self, channel: WsChannelType) -> Result<(WsSink, WsReader)> {
        if self.is_connected() {
            self.close().await?;
            return Err(error::Error::Other("WebSocket is already connected"));
        }
        let request = self.request()?;
        println!("Connecting to WebSocket: {:?}", request.headers());
        let (ws_stream, _) = tokio_tungstenite::connect_async(request.uri()).await?;
        let (mut write, read) = ws_stream.split();

        if channel == WsChannelType::Private {
            write
                .send(Message::Text(self.login_request()?.into()))
                .await?;
        }
        self.set_connected_state(true);
        Ok((write, read))
    }

    async fn close(&mut self) -> Result<()> {
        self.set_connected_state(false);
        Ok(())
    }
    fn is_connected(&self) -> bool;
    async fn reconnect(&mut self) -> Result<()>;

    fn set_connected_state(&mut self, state: bool);
}

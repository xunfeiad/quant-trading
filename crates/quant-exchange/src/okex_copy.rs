// use std::collections::HashMap;
// use std::sync::Arc;
// use std::time::Duration;

// use crate::error::{Error, Result};
// use crate::schema::okex::channel::WebSocketChannelType;
// use crate::{WsReader, WsSink};
// use base64::{Engine, engine::general_purpose};
// use bytes::Bytes;
// use flume::Sender;
// use futures_util::{SinkExt, StreamExt};
// use hmac::{Hmac, Mac};
// use hyper::{HeaderMap, Method};
// use quant_config::{Credentials, Protocol, get_credentials};
// use quant_schema::Exchange;
// use quant_schema::okex::ws::arg::{LoginArgs, OkexWsClientBodys, WsArg, WsPrivateArg};
// use quant_schema::okex::ws::event::{WsRequestMessage, WsResponseMessage};
// use quant_schema::okex::ws::operation::Operation;
// use reqwest::Client;
// use tokio::select;
// use tokio::sync::RwLock;
// use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

// pub type ConnId = String;

// pub type WsUsers = HashMap<ConnId, OkexConnection>;

// const RETRY_COUNT: usize = 3;
// const TIMEOUT_DURATION: u64 = 5; // 5 seconds

// #[derive(Clone, Debug)]
// pub struct WebSocketConnection {
//     pub message_sender: Option<Arc<RwLock<WsSink>>>,
//     pub message_receiver: Option<Arc<RwLock<WsReader>>>,
// }

// impl WebSocketConnection {
//     pub fn new() -> Self {
//         Self {
//             message_sender: None,
//             message_receiver: None,
//         }
//     }

//     pub fn is_active(&self) -> bool {
//         self.message_sender.is_some() && self.message_receiver.is_some()
//     }

//     pub fn close(&mut self) {
//         self.message_sender.take();
//         self.message_receiver.take();
//     }
// }

// #[derive(Clone)]
// pub struct WebSocketConnectionManager {
//     pub public_connection: WebSocketConnection,
//     pub private_connection: WebSocketConnection,
//     pub business_connection: WebSocketConnection,
// }

// impl Drop for WebSocketConnectionManager {
//     fn drop(&mut self) {
//         self.public_connection.message_sender.take();
//         self.private_connection.message_sender.take();
//         self.business_connection.message_sender.take();
//     }
// }

// impl WebSocketConnectionManager {
//     pub fn new(
//         public_connection: WebSocketConnection,
//         private_connection: WebSocketConnection,
//         business_connection: WebSocketConnection,
//     ) -> Self {
//         Self {
//             public_connection,
//             private_connection,
//             business_connection,
//         }
//     }

//     pub fn get_connection(
//         &mut self,
//         channel_type: &WebSocketChannelType,
//     ) -> Result<&mut WebSocketConnection> {
//         match channel_type {
//             WebSocketChannelType::Public => Ok(&mut self.public_connection),
//             WebSocketChannelType::Private => Ok(&mut self.private_connection),
//             WebSocketChannelType::Business => Ok(&mut self.business_connection),
//             WebSocketChannelType::Error => {
//                 Err(Error::Other("Error channel is not supported".into()))
//             }
//         }
//     }

//     pub fn get_connection_ref(
//         &self,
//         channel_type: &WebSocketChannelType,
//     ) -> Result<&WebSocketConnection> {
//         match channel_type {
//             WebSocketChannelType::Public => Ok(&self.public_connection),
//             WebSocketChannelType::Private => Ok(&self.private_connection),
//             WebSocketChannelType::Business => Ok(&self.business_connection),
//             WebSocketChannelType::Error => {
//                 Err(Error::Other("Error channel is not supported".into()))
//             }
//         }
//     }

//     pub fn health_check(&self) -> ConnectionHealthCheck {
//         ConnectionHealthCheck {
//             public_active: self.public_connection.is_active(),
//             private_active: self.private_connection.is_active(),
//             business_active: self.business_connection.is_active(),
//         }
//     }
// }

// pub struct ConnectionHealthCheck {
//     pub public_active: bool,
//     pub private_active: bool,
//     pub business_active: bool,
// }

// impl ConnectionHealthCheck {
//     pub fn all_active(&self) -> bool {
//         self.public_active && self.private_active && self.business_active
//     }

//     pub fn any_active(&self) -> bool {
//         self.public_active || self.private_active || self.business_active
//     }
// }

// #[derive(Clone)]
// pub struct MessageQueueManager {
//     /// 公开市场数据消息队列
//     pub public_message_queue: Sender<WsResponseMessage>,
//     /// 私有账户数据消息队列
//     pub private_message_queue: Sender<WsResponseMessage>,
//     /// 业务数据消息队列
//     pub business_message_queue: Sender<WsResponseMessage>,
//     /// 错误消息队列
//     pub error_message_queue: Sender<WsResponseMessage>,
// }

// impl MessageQueueManager {
//     pub fn new(
//         public_message_queue: Sender<WsResponseMessage>,
//         private_message_queue: Sender<WsResponseMessage>,
//         business_message_queue: Sender<WsResponseMessage>,
//         error_message_queue: Sender<WsResponseMessage>,
//     ) -> Self {
//         Self {
//             public_message_queue,
//             private_message_queue,
//             business_message_queue,
//             error_message_queue,
//         }
//     }

//     pub fn get_queue(
//         &self,
//         channel_type: &WebSocketChannelType,
//     ) -> Result<&Sender<WsResponseMessage>> {
//         match channel_type {
//             WebSocketChannelType::Public => Ok(&self.public_message_queue),
//             WebSocketChannelType::Private => Ok(&self.private_message_queue),
//             WebSocketChannelType::Business => Ok(&self.business_message_queue),
//             WebSocketChannelType::Error => Ok(&self.error_message_queue),
//         }
//     }

//     pub async fn send_message(
//         &self,
//         channel_type: &WebSocketChannelType,
//         message: WsResponseMessage,
//     ) -> Result<()> {
//         let queue = self.get_queue(channel_type)?;
//         queue.send_async(message).await?;
//         Ok(())
//     }

//     pub fn get_queue_stas(&self) -> QueueStatistics {
//         QueueStatistics {
//             public_queue_size: self.public_message_queue.len(),
//             private_queue_size: self.private_message_queue.len(),
//             business_queue_size: self.business_message_queue.len(),
//             error_queue_size: self.error_message_queue.len(),
//         }
//     }
// }

// pub struct QueueStatistics {
//     pub public_queue_size: usize,
//     pub private_queue_size: usize,
//     pub business_queue_size: usize,
//     pub error_queue_size: usize,
// }

// pub struct OkexWebSocketClient {
//     pub connection_id: ConnId,
//     pub connection_manager: WebSocketConnectionManager,
//     pub message_queue_manager: MessageQueueManager,
//     pub authentication_status: AuthenticationStatus,
// }

// #[derive(Clone, Debug)]
// pub struct AuthenticationStatus {
//     pub is_authenticated: bool,
//     pub authentication_at: Option<std::time::Instant>,
//     pub auth_failure_count: usize,
//     pub last_auth_attempt: Option<std::time::Instant>,
// }

// impl Default for AuthenticationStatus {
//     fn default() -> Self {
//         Self {
//             is_authenticated: false,
//             authentication_at: None,
//             auth_failure_count: 0,
//             last_auth_attempt: None,
//         }
//     }
// }

// impl AuthenticationStatus {
//     pub fn mark_authenticated(&mut self) {
//         self.is_authenticated = true;
//         self.authentication_at = Some(std::time::Instant::now());
//         self.auth_failure_count = 0;
//     }

//     pub fn mark_auth_failed(&mut self) {
//         self.is_authenticated = false;
//         self.auth_failure_count += 1;
//         self.last_auth_attempt = Some(std::time::Instant::now());
//     }

//     pub fn reset(&mut self) {
//         *self = AuthenticationStatus::default();
//     }

//     pub fn show_retry_auth(&self, max_failure: usize) -> bool {
//         !self.is_authenticated && self.auth_failure_count < max_failure
//     }
// }

// pub struct ClientStatus {
//     pub connection_id: ConnId,
//     pub connection_health: ConnectionHealthCheck,
//     pub queue_statistics: QueueStatistics,
//     pub auth_status: AuthenticationStatus,
// }

// impl OkexWebSocketClient {
//     pub fn new(
//         connection_id: ConnId,
//         connection_manager: WebSocketConnectionManager,
//         message_queue_manager: MessageQueueManager,
//     ) -> Self {
//         Self {
//             connection_id,
//             connection_manager,
//             message_queue_manager,
//             authentication_status: AuthenticationStatus::default(),
//         }
//     }

//     pub fn get_client_status(&self) -> ClientStatus {
//         ClientStatus {
//             connection_id: self.connection_id.clone(),
//             connection_health: self.connection_manager.health_check(),
//             queue_statistics: self.message_queue_manager.get_queue_stas(),
//             auth_status: self.authentication_status.clone(),
//         }
//     }
//     // 获取 Okex 的凭证
//     pub fn credentials(&self) -> Result<Credentials> {
//         let credentials = get_credentials(&Exchange::Okex)?;
//         Ok(credentials)
//     }

//     pub async fn initializes(&mut self) -> Result<()> {
//         let public = self
//             .get_writer_and_reader(&WebSocketChannelType::Public)
//             .await?;
//         let private = self
//             .get_writer_and_reader(&WebSocketChannelType::Private)
//             .await?;
//         let business = self
//             .get_writer_and_reader(&WebSocketChannelType::Business)
//             .await?;
//         self.wss = OkexEndpoints {
//             public,
//             private,
//             business,
//         };
//         Ok(())
//     }

//     // 登陆
//     pub async fn login(&mut self) -> Result<()> {
//         let credentials = get_credentials(&Exchange::Okex)?;

//         let timestamp = chrono::Utc::now().timestamp();

//         let sign = gen_signature(
//             &Method::GET,
//             "/users/self/verify",
//             "",
//             timestamp as f64,
//             &credentials.secret_key,
//         )?;
//         if self.is_authenticated {
//             return Ok(());
//         }
//         let arg = WsArg::new_private(WsPrivateArg::Login(LoginArgs {
//             api_key: credentials.api_key.clone(),
//             passphrase: credentials.passphrase.clone(),
//             timestamp: timestamp as f64,
//             sign,
//         }));
//         let body = self.build_body(&Operation::Login, &[arg])?;
//         self.wss
//             .private
//             .writer
//             .as_mut()
//             .ok_or(Error::Other("No WebSocket writer available".into()))?
//             .write()
//             .await
//             .send(Message::Text(Utf8Bytes::from(body)))
//             .await?;
//         self.is_authenticated = true;
//         Ok(())
//     }

//     // 构建请求体
//     pub fn build_body(&self, operation: &Operation, args: &[WsArg]) -> Result<String> {
//         let args: OkexWsClientBodys = self.build_args(args, operation)?;
//         let mut new_args = Vec::new();
//         if args.public_body.len() > 0 {
//             new_args.extend(args.public_body);
//         }
//         if args.private_body.len() > 0 {
//             new_args.extend(args.private_body);
//         }
//         if args.business_body.len() > 0 {
//             return Err(Error::Other(
//                 "Business arguments are not supported in this context".into(),
//             ));
//         }
//         let body = WsRequestMessage {
//             id: None,
//             op: operation.clone(),
//             args: new_args,
//         };
//         Ok(serde_json::to_string(&body)?)
//     }

//     // 构建请求参数
//     pub fn build_args(&self, args: &[WsArg], operation: &Operation) -> Result<OkexWsClientBodys> {
//         if operation == &Operation::Login {
//             if args.len() != 1 {
//                 return Err(Error::Other(
//                     "Login operation requires exactly one argument".into(),
//                 ));
//             }
//         }
//         let mut client_body = OkexWsClientBodys::new();

//         for arg in args {
//             match arg {
//                 WsArg::Public(public_arg) => {
//                     client_body.public_body.push(public_arg.as_value()?);
//                 }
//                 WsArg::Private(private_arg) => match private_arg {
//                     WsPrivateArg::Login(_) => {
//                         if !self.is_authenticated {
//                             client_body.private_body.push(private_arg.as_value()?);
//                         }
//                     }
//                     _ => {
//                         client_body.private_body.push(private_arg.as_value()?);
//                     }
//                 },
//                 WsArg::Business => panic!("Business argument is not supported in this context"),
//             };
//         }

//         // 验证参数数量
//         let mut validate_arg_category_count = 0;
//         if !client_body.public_body.is_empty() {
//             validate_arg_category_count += 1;
//         }
//         if !client_body.private_body.is_empty() {
//             validate_arg_category_count += 1;
//         }
//         if !client_body.business_body.is_empty() {
//             validate_arg_category_count += 1;
//         }
//         if validate_arg_category_count > 1 {
//             return Err(Error::Other("Too many argument categories provided".into()));
//         }

//         Ok(client_body)
//     }

//     // 获取 WebSocket 的写入器和读取器
//     pub async fn get_writer_and_reader(
//         &self,
//         channel_type: &WebSocketChannelType,
//     ) -> Result<WebSocketEndpoint> {
//         let (ws_stream, _) =
//             tokio_tungstenite::connect_async(self.websocket_url(channel_type)?).await?;
//         let (writer, reader) = ws_stream.split();
//         Ok(WebSocketEndpoint {
//             writer: Some(Arc::new(RwLock::new(writer))),
//             reader: Some(Arc::new(RwLock::new(reader))),
//         })
//     }

//     // 发送消息
//     pub async fn send_message(
//         &mut self,
//         operation: &Operation,
//         args: &[WsArg],
//         channel_type: &WebSocketChannelType,
//     ) -> Result<()> {
//         let body = self.build_body(operation, args)?;
//         let need_login = *channel_type == WebSocketChannelType::Private && !self.is_authenticated;

//         if need_login {
//             self.login().await?;
//         }
//         let writer = &mut self.wss.get(channel_type)?.writer;

//         writer
//             .as_mut()
//             .ok_or(Error::Other("No WebSocket writer available".into()))?
//             .write()
//             .await
//             .send(Message::Text(Utf8Bytes::from(body)))
//             .await?;

//         Ok(())
//     }

//     pub async fn ping(&self, writer: &Arc<RwLock<WsSink>>) -> Result<()> {
//         writer
//             .write()
//             .await
//             .send(Message::Ping(Bytes::new()))
//             .await?;

//         Ok(())
//     }

//     pub async fn handle_message_by_channel(
//         &self,
//         ws: WebSocketEndpoint,
//         queue: &mut flume::Sender<WsResponseMessage>,
//         channel_type: &WebSocketChannelType,
//     ) -> Result<()> {
//         println!("Handling  message for channel: {:?}", channel_type);
//         let mut ticker = tokio::time::interval(Duration::from_secs(TIMEOUT_DURATION));
//         let reader = ws
//             .reader
//             .as_ref()
//             .ok_or(Error::Other("No public WebSocket reader available".into()))?
//             .clone();

//         let writer = ws
//             .writer
//             .as_ref()
//             .ok_or(Error::Other("No public WebSocket writer available".into()))?
//             .clone();
//         loop {
//             select! {
//                 _ = self.process_message(&reader, queue) => {

//                 },
//                 _ = ticker.tick() => {
//                     if let Err(e) = self.ping(&writer).await {
//                         eprintln!("Error pinging public channel: {}", e);
//                         return Err(e);
//                     }
//                 }
//             }
//         }
//     }

//     // 接收消息
//     pub async fn receive_message(&self, queue: &mut OkexWsQueue) -> Result<()> {
//         let public_reader = self.wss.public.clone();
//         let private_reader = self.wss.private.clone();
//         let business_reader = self.wss.business.clone();
//         tokio::try_join!(
//             self.handle_message_by_channel(
//                 public_reader,
//                 &mut queue.public_queue,
//                 &WebSocketChannelType::Public
//             ),
//             self.handle_message_by_channel(
//                 private_reader,
//                 &mut queue.private_queue,
//                 &WebSocketChannelType::Private
//             ),
//             self.handle_message_by_channel(
//                 business_reader,
//                 &mut queue.business_queue,
//                 &WebSocketChannelType::Business
//             ),
//         )?;
//         Ok(())
//     }

//     pub async fn process_message(
//         &self,
//         reader: &Arc<RwLock<WsReader>>,
//         queue: &mut flume::Sender<WsResponseMessage>,
//     ) -> Result<()> {
//         let response = self.receive_ws_response(reader).await?;
//         // self.filter_error_response(&response).await?;
//         send_to_spec_queue(queue, response).await?;
//         Ok(())
//     }
//     pub async fn filter_error_response(&mut self, response: &WsResponseMessage) -> Result<()> {
//         if response.code == "60011" {
//             self.login().await?;
//         }
//         if response.code == "50110" {
//             return Err(Error::Other("Ip is not in linking trusted IP addresses."));
//         }
//         Ok(())
//     }

//     pub async fn receive_ws_response(
//         &self,
//         ws: &Arc<RwLock<WsReader>>,
//     ) -> Result<WsResponseMessage> {
//         let mut reader = ws.write().await;
//         let message = match reader.next().await {
//             Some(Ok(msg)) => msg,
//             Some(Err(e)) => {
//                 return Err(e.into());
//             }
//             None => {
//                 return Err(Error::Other("WebSocket connection closed by server".into()));
//             }
//         };
//         let response = match message {
//             Message::Text(text) => serde_json::from_str::<WsResponseMessage>(text.as_str())?,
//             Message::Binary(_) => {
//                 return Err(Error::Other("Binary messages are not supported".into()));
//             }
//             Message::Close(_) => {
//                 eprintln!("⚠️ WebSocket connection closed by server");
//                 return Err(Error::Other("WebSocket connection closed by server".into()));
//             }
//             _ => {
//                 return Err(Error::Other("Unsupported message type".into()));
//             }
//         };
//         Ok(response)
//     }

//     pub async fn subscribe(
//         &mut self,
//         operation: &Operation,
//         args: &[WsArg],
//         channel_type: &WebSocketChannelType,
//     ) -> Result<()> {
//         self.send_message(operation, args, channel_type).await?;
//         Ok(())
//     }

//     pub async fn unsubscribe(
//         &mut self,
//         operation: &Operation,
//         args: &[WsArg],
//         channel_type: &WebSocketChannelType,
//     ) -> Result<()> {
//         self.send_message(operation, args, channel_type).await?;
//         Ok(())
//     }

//     pub fn websocket_url(&self, channel_type: &WebSocketChannelType) -> Result<String> {
//         let credentials = self.credentials()?;
//         if credentials.protocol != Some(Protocol::WSS) {
//             return Err(Error::Other("The protocol is not WSS"));
//         }

//         let uri = credentials
//             .ws_urls
//             .get(0)
//             .ok_or_else(|| Error::Other("No WebSocket URL found in credentials"))?;

//         let uri_suffix = channel_type.as_str();
//         let url = format!("{}{}", uri, uri_suffix);
//         Ok(url)
//     }
// }
// pub async fn send_to_spec_queue(
//     queue: &mut flume::Sender<WsResponseMessage>,
//     data: WsResponseMessage,
// ) -> Result<()> {
//     queue.send_async(data).await?;
//     Ok(())
// }

// pub fn sign_request(
//     credentials: &Credentials,
//     method: &Method,
//     path: &str,
//     body: &str,
// ) -> Result<HeaderMap> {
//     let mut headers = HeaderMap::new();
//     let timestamp = chrono::Utc::now().timestamp();
//     let sign = gen_signature(
//         method,
//         path,
//         body,
//         timestamp as f64,
//         &credentials.secret_key,
//     )?;
//     headers.insert("OK-ACCESS-KEY", credentials.api_key.parse()?);
//     headers.insert("OK-ACCESS-SIGN", sign.parse().unwrap());
//     headers.insert("OK-ACCESS-TIMESTAMP", timestamp.to_string().parse()?);
//     if credentials.use_testnet.unwrap_or(false) && credentials.protocol == Some(Protocol::WSS) {
//         headers.insert("x-simulated-trading", "1".parse()?);
//     }
//     Ok(headers)
// }

// pub fn gen_signature(
//     method: &Method,
//     path: &str,
//     body: &str,
//     timestamp: f64,
//     secret: &str,
// ) -> Result<String> {
//     let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes())?;
//     let merged_string = format!("{}{}{}{}", timestamp, method.as_str(), path, body);
//     mac.update(merged_string.as_bytes());
//     let result = mac.finalize();
//     let signature = general_purpose::STANDARD.encode(result.into_bytes());
//     Ok(signature)
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use quant_schema::okex::order::InstrumentType;
//     use quant_schema::okex::ws::arg::{OrderArg, WsArg, WsPrivateArg};
//     use quant_schema::okex::ws::event::WsRequestMessage;
//     use quant_schema::okex::ws::operation::Operation;
//     use serde_json::json;
//     #[test]
//     pub fn test_build_request_body() {
//         let okex = OkexConnection {
//             conn_id: "test_conn_id".into(),
//             is_authenticated: false,
//             wss: OkexEndpoints {
//                 public: PublicWebSocketEndpoint {
//                     writer: None,
//                     reader: None,
//                 },
//                 private: PrivateWebSocketEndpoint {
//                     writer: None,
//                     reader: None,
//                 },
//                 business: BusinessWebSocketEndpoint {
//                     writer: None,
//                     reader: None,
//                 },
//             },
//         };
//         let args = [WsArg::new_private(WsPrivateArg::Order(OrderArg {
//             inst_type: InstrumentType::SPOT,
//             inst_id: Some("BTC-USDT".into()),
//             inst_family: Some("  ".into()),
//         }))];

//         let body = okex.build_body(&Operation::Subscribe, &args);

//         assert!(body.is_ok());
//         let ws_event_data: WsRequestMessage = serde_json::from_str(&body.unwrap()).unwrap();
//         let expected_event = WsRequestMessage {
//             id: None,
//             op: Operation::Subscribe,
//             args: vec![
//                 json!({"channel": "orders", "instType": "SPOT", "instId": "BTC-USDT", "instFamily": "  "}),
//             ],
//         };
//         assert_eq!(ws_event_data, expected_event);
//     }

//     #[tokio::test]
//     pub async fn test_public_ws_connection() {
//         let mut okex = OkexConnection {
//             conn_id: "test_conn_id".into(),
//             is_authenticated: false,
//             wss: OkexEndpoints {
//                 public: PublicWebSocketEndpoint {
//                     writer: None,
//                     reader: None,
//                 },
//                 private: PrivateWebSocketEndpoint {
//                     writer: None,
//                     reader: None,
//                 },
//                 business: BusinessWebSocketEndpoint {
//                     writer: None,
//                     reader: None,
//                 },
//             },
//         };
//         let args = [WsArg::new_private(WsPrivateArg::Order(OrderArg {
//             inst_type: InstrumentType::SPOT,
//             inst_id: Some("BTC-USDT".into()),
//             inst_family: Some("  ".into()),
//         }))];

//         okex.initializes().await.unwrap();
//         // okex
//         //     .login()
//         //     .await
//         //     .unwrap();
//         let mut queue = OkexWsQueue {
//             public_queue: flume::unbounded().0,
//             private_queue: flume::unbounded().0,
//             business_queue: flume::unbounded().0,
//             error_queue: flume::unbounded().0,
//         };
//         okex.send_message(&Operation::Subscribe, &args, &WebSocketChannelType::Public)
//             .await
//             .unwrap();
//         if let Ok(e) = okex.receive_message(&mut queue).await {
//             println!("{:?}", e);
//             assert!(
//                 false,
//                 "Should not receive messages in public channel without authentication"
//             );
//         }
//     }
// }

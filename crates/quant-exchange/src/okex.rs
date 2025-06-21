use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use crate::error::{Error, Result};
use crate::schema::okex::channel::WebSocketChannelType;
use crate::{WsReader, WsSink};
use base64::{Engine, engine::general_purpose};
use bytes::Bytes;
use flume::{Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use hyper::Method;
use parking_lot::RwLock;
use quant_config::{Credentials, Protocol, get_credentials};
use quant_schema::Exchange;
use quant_schema::okex::ws::arg::{LoginArgs, OkexWsClientBodys, WsArg, WsPrivateArg};
use quant_schema::okex::ws::event::{WsRequestMessage, WsResponseMessage};
use quant_schema::okex::ws::operation::Operation;
use tokio::select;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use tracing::{debug, error, info, warn};

pub type ConnId = String;

const RETRY_COUNT: usize = 3;
const TIMEOUT_DURATION: Duration = Duration::from_secs(5);
const PING_INTERVAL: Duration = Duration::from_secs(30);
const RECONNECT_DELAY: Duration = Duration::from_secs(5);
const MAX_AUTH_FAILURES: usize = 3;
const MAX_AUTHENTICATION_INTERVAL: Duration = Duration::from_secs(60);

/// WebSocket连接状态
#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Authenticated,
    Error(String),
}

/// 单个WebSocket连接的封装
#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    pub sender: Option<Arc<RwLock<WsSink>>>,
    pub receiver: Option<Arc<RwLock<WsReader>>>,
    state: ConnectionState,
    last_ping: Option<Instant>,
    reconnect_count: usize,
}

impl WebSocketConnection {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
            state: ConnectionState::Disconnected,
            last_ping: None,
            reconnect_count: 0,
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self.state,
            ConnectionState::Connected | ConnectionState::Authenticated
        )
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self.state, ConnectionState::Authenticated)
    }

    pub fn set_connection(&mut self, sender: WsSink, receiver: WsReader) {
        self.sender = Some(Arc::new(RwLock::new(sender)));
        self.receiver = Some(Arc::new(RwLock::new(receiver)));
        self.state = ConnectionState::Connected;
        self.last_ping = Some(Instant::now());
        info!("WebSocket connection established");
    }

    pub fn set_authenticated(&mut self) {
        if self.is_connected() {
            self.state = ConnectionState::Authenticated;
            info!("WebSocket connection authenticated");
        }
    }

    pub fn disconnect(&mut self) {
        self.sender.take();
        self.receiver.take();
        self.state = ConnectionState::Disconnected;
        self.last_ping = None;
        info!("WebSocket connection disconnected");
    }

    pub fn set_error(&mut self, error: String) {
        self.state = ConnectionState::Error(error);
        self.sender.take();
        self.receiver.take();
        self.last_ping = None;
        self.reconnect_count = 0; // 重置重连计数

        error!(
            "WebSocket connection error: {}",
            self.get_error_message().unwrap_or(&"".to_string())
        );
    }

    pub fn get_error_message(&self) -> Option<&String> {
        if let ConnectionState::Error(msg) = &self.state {
            Some(msg)
        } else {
            None
        }
    }

    pub fn should_reconnect(&self) -> bool {
        matches!(
            self.state,
            ConnectionState::Error(_) | ConnectionState::Disconnected
        ) && self.reconnect_count < RETRY_COUNT
    }

    pub fn increment_reconnect_count(&mut self) {
        self.reconnect_count += 1;
    }

    pub fn reset_reconnect_count(&mut self) {
        self.reconnect_count = 0;
    }

    pub fn get_reader(&self) -> Option<Arc<RwLock<WsReader>>> {
        self.receiver.clone()
    }
}

/// 连接管理器，管理所有类型的WebSocket连接
#[derive(Clone)]
pub struct WebSocketConnectionManager {
    public_connection: WebSocketConnection,
    private_connection: WebSocketConnection,
    business_connection: WebSocketConnection,
}

impl WebSocketConnectionManager {
    pub fn new() -> Self {
        Self {
            public_connection: WebSocketConnection::new(),
            private_connection: WebSocketConnection::new(),
            business_connection: WebSocketConnection::new(),
        }
    }

    pub fn get_connection_mut(
        &mut self,
        channel_type: &WebSocketChannelType,
    ) -> Result<&mut WebSocketConnection> {
        match channel_type {
            WebSocketChannelType::Public => Ok(&mut self.public_connection),
            WebSocketChannelType::Private => Ok(&mut self.private_connection),
            WebSocketChannelType::Business => Ok(&mut self.business_connection),
            WebSocketChannelType::Error => {
                Err(Error::Other("Error channel is not supported".into()))
            }
        }
    }

    pub fn get_connection(
        &self,
        channel_type: &WebSocketChannelType,
    ) -> Result<&WebSocketConnection> {
        match channel_type {
            WebSocketChannelType::Public => Ok(&self.public_connection),
            WebSocketChannelType::Private => Ok(&self.private_connection),
            WebSocketChannelType::Business => Ok(&self.business_connection),
            WebSocketChannelType::Error => {
                Err(Error::Other("Error channel is not supported".into()))
            }
        }
    }

    pub fn health_check(&self) -> ConnectionHealthCheck {
        ConnectionHealthCheck {
            public_active: self.public_connection.is_connected(),
            private_active: self.private_connection.is_connected(),
            business_active: self.business_connection.is_connected(),
        }
    }

    pub fn disconnect_all(&mut self) {
        self.public_connection.disconnect();
        self.private_connection.disconnect();
        self.business_connection.disconnect();
    }
}

/// 连接健康检查结果
#[derive(Debug, Clone)]
pub struct ConnectionHealthCheck {
    pub public_active: bool,
    pub private_active: bool,
    pub business_active: bool,
}

impl ConnectionHealthCheck {
    pub fn all_active(&self) -> bool {
        self.public_active && self.private_active && self.business_active
    }

    pub fn any_active(&self) -> bool {
        self.public_active || self.private_active || self.business_active
    }
}

/// 消息队列管理器
#[derive(Clone)]
pub struct MessageQueueManager {
    public_sender: Sender<WsResponseMessage>,
    private_sender: Sender<WsResponseMessage>,
    business_sender: Sender<WsResponseMessage>,
    error_sender: Sender<WsResponseMessage>,
}

impl MessageQueueManager {
    pub fn new() -> (Self, MessageQueueReceivers) {
        let (public_sender, public_rx) = flume::unbounded();
        let (private_sender, private_rx) = flume::unbounded();
        let (business_sender, business_rx) = flume::unbounded();
        let (error_sender, error_rx) = flume::unbounded();

        let manager = Self {
            public_sender,
            private_sender,
            business_sender,
            error_sender,
        };

        let receivers = MessageQueueReceivers {
            public_receiver: public_rx,
            private_receiver: private_rx,
            business_receiver: business_rx,
            error_receiver: error_rx,
        };

        (manager, receivers)
    }

    pub fn get_sender(
        &self,
        channel_type: &WebSocketChannelType,
    ) -> Result<&Sender<WsResponseMessage>> {
        match channel_type {
            WebSocketChannelType::Public => Ok(&self.public_sender),
            WebSocketChannelType::Private => Ok(&self.private_sender),
            WebSocketChannelType::Business => Ok(&self.business_sender),
            WebSocketChannelType::Error => Ok(&self.error_sender),
        }
    }

    pub async fn send_message(
        &self,
        channel_type: &WebSocketChannelType,
        message: WsResponseMessage,
    ) -> Result<()> {
        let sender = self.get_sender(channel_type)?;
        sender.send_async(message).await?;
        Ok(())
    }
}

/// 消息队列接收端
pub struct MessageQueueReceivers {
    pub public_receiver: Receiver<WsResponseMessage>,
    pub private_receiver: Receiver<WsResponseMessage>,
    pub business_receiver: Receiver<WsResponseMessage>,
    pub error_receiver: Receiver<WsResponseMessage>,
}

/// 认证状态管理
#[derive(Clone, Debug)]
pub struct AuthenticationStatus {
    pub is_authenticated: bool,
    pub authenticated_at: Option<Instant>,
    pub failure_count: usize,
    pub last_attempt: Option<Instant>,
}

impl Default for AuthenticationStatus {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            authenticated_at: None,
            failure_count: 0,
            last_attempt: None,
        }
    }
}

impl AuthenticationStatus {
    pub fn mark_authenticated(&mut self) {
        self.is_authenticated = true;
        self.authenticated_at = Some(Instant::now());
        self.failure_count = 0;
        info!("Authentication successful");
    }

    pub fn mark_failed(&mut self) {
        self.is_authenticated = false;
        self.failure_count += 1;
        self.last_attempt = Some(Instant::now());
        warn!(
            "Authentication failed, failure count: {}",
            self.failure_count
        );
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn should_retry(&self) -> bool {
        !self.is_authenticated && self.failure_count < MAX_AUTH_FAILURES
    }

    pub fn can_attempt_auth(&self) -> bool {
        self.last_attempt
            .map(|last| last.elapsed() > MAX_AUTHENTICATION_INTERVAL)
            .unwrap_or(true)
    }
}

#[derive(Debug, Clone)]
pub struct MessageQueueState {
    pub public_queue_size: usize,
    pub private_queue_size: usize,
    pub business_queue_size: usize,
    pub error_queue_size: usize,
}
/// 客户端状态信息
#[derive(Debug)]
pub struct ClientStatus {
    pub connection_id: ConnId,
    pub connection_health: ConnectionHealthCheck,
    pub auth_status: AuthenticationStatus,
    pub message_queue_state: MessageQueueState,
}

/// 主要的OkexWebSocketClient实现
#[derive(Clone)]
pub struct OkexWebSocketClient {
    pub connection_id: ConnId,
    pub connection_manager: WebSocketConnectionManager,
    pub message_queue_manager: MessageQueueManager,
    pub auth_status: Arc<RwLock<AuthenticationStatus>>,
    pub credentials: Option<Credentials>,
}

impl OkexWebSocketClient {
    pub fn new(connection_id: ConnId) -> (Self, MessageQueueReceivers) {
        let (message_queue_manager, receivers) = MessageQueueManager::new();

        let client = Self {
            connection_id,
            connection_manager: WebSocketConnectionManager::new(),
            message_queue_manager,
            auth_status: Arc::new(RwLock::new(AuthenticationStatus::default())),
            credentials: None,
        };

        (client, receivers)
    }

    /// 初始化客户端，建立所有连接
    pub async fn initialize(&mut self) -> Result<()> {
        info!(
            "Initializing OkexWebSocketClient with ID: {}",
            self.connection_id
        );

        // 加载凭证
        self.credentials = Some(get_credentials(&Exchange::Okex)?);

        // 建立所有连接
        self.connect_channel(&WebSocketChannelType::Public).await?;
        self.connect_channel(&WebSocketChannelType::Private).await?;
        self.connect_channel(&WebSocketChannelType::Business)
            .await?;

        info!("All WebSocket connections established");
        Ok(())
    }

    /// 建立指定类型的连接
    async fn connect_channel(&mut self, channel_type: &WebSocketChannelType) -> Result<()> {
        let url = self.get_websocket_url(channel_type)?;
        debug!("Connecting to {} channel: {}", channel_type.as_str(), url);

        let connection = self.connection_manager.get_connection_mut(channel_type)?;
        connection.state = ConnectionState::Connecting;

        match tokio_tungstenite::connect_async(&url).await {
            Ok((ws_stream, _)) => {
                let (sink, stream) = ws_stream.split();
                connection.set_connection(sink, stream);
                connection.reset_reconnect_count();
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to connect to {}: {}", channel_type.as_str(), e);
                connection.set_error(error_msg.clone());
                Err(e.into())
            }
        }
    }

    /// 认证私有连接
    pub async fn authenticate(&mut self) -> Result<()> {
        if self.auth_status.read().is_authenticated {
            return Ok(());
        }

        if !self.auth_status.read().can_attempt_auth() {
            return Err(Error::Other(
                "Too many authentication attempts, please wait".into(),
            ));
        }

        let credentials = self
            .credentials
            .as_ref()
            .ok_or_else(|| Error::Other("Credentials not loaded".into()))?;

        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs_f64();
        let sign = self.generate_signature(
            &Method::GET,
            "/users/self/verify",
            "",
            timestamp,
            &credentials.secret_key,
        )?;

        let login_arg = WsArg::new_private(WsPrivateArg::Login(LoginArgs {
            api_key: credentials.api_key.clone(),
            passphrase: credentials.passphrase.clone(),
            timestamp,
            sign,
        }));

        let body = self.build_request_body(&Operation::Login, &[login_arg])?;

        // 发送认证请求
        let connection = self
            .connection_manager
            .get_connection(&WebSocketChannelType::Private)?;
        if let Some(sender) = &connection.sender {
            sender
                .write()
                .send(Message::Text(Utf8Bytes::from(serde_json::to_string(
                    &body,
                )?)))
                .await?;

            info!("Authentication request sent");
            self.auth_status.write().last_attempt = Some(Instant::now());
        } else {
            return Err(Error::Other("Private connection not available".into()));
        }

        Ok(())
    }

    /// 启动消息处理循环
    pub async fn start_message_handling(&mut self) -> Result<()> {
        info!("Starting message handling for all channels");
        // 并发处理所有通道
        let (pub_res, priv_res, bus_res) = tokio::join!(
            self.handle_channel_messages(&WebSocketChannelType::Public),
            self.handle_channel_messages(&WebSocketChannelType::Private),
            self.handle_channel_messages(&WebSocketChannelType::Business),
        );

        if let Err(e) = pub_res {
            error!("Public channel exited with error: {:?}", e);
            self.connection_manager
                .get_connection_mut(&WebSocketChannelType::Public)?
                .set_error(format!("Public channel error: {}", e));
        }
        if let Err(e) = priv_res {
            error!("Private channel exited with error: {:?}", e);
            self.connection_manager
                .get_connection_mut(&WebSocketChannelType::Private)?
                .set_error(format!("Private channel error: {}", e));
        }
        if let Err(e) = bus_res {
            error!("Business channel exited with error: {:?}", e);
            self.connection_manager
                .get_connection_mut(&WebSocketChannelType::Business)?
                .set_error(format!("Business channel error: {}", e));
        }

        Ok(())
    }

    /// 处理单个通道的消息
    async fn handle_channel_messages(&self, channel_type: &WebSocketChannelType) -> Result<()> {
        let connection = self.connection_manager.get_connection(channel_type)?;

        if let (Some(reader), Some(writer)) = (&connection.receiver, &connection.sender) {
            let mut ping_interval = tokio::time::interval(PING_INTERVAL);

            loop {
                select! {
                    // 处理接收到的消息
                    result = self.receive_message(&reader, &writer) => {
                        match result {
                            Ok(message) => {
                                self.handle_received_message(channel_type, message).await?;
                            }
                            Err(e) => {
                                error!("Error receiving message from {}: {}", channel_type.as_str(), e);
                                return Err(e);
                            }
                        }
                    }

                    // 定期发送ping
                    _ = ping_interval.tick() => {
                        if let Err(e) = self.send_ping(&writer).await {
                            error!("Error sending ping to {}: {}", channel_type.as_str(), e);
                            return Err(e);
                        }
                    }
                    _ = async move {
                        let status = self.get_status();
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        error!("Checking status for {:?}", status);
                    } => {}
                }
            }
        } else {
            Err(Error::OtherError(format!(
                "{} connection not available",
                channel_type.as_str()
            )))
        }
    }

    /// 接收单个消息
    pub async fn receive_message(
        &self,
        reader: &Arc<RwLock<WsReader>>,
        writer: &Arc<RwLock<WsSink>>,
    ) -> Result<WsResponseMessage> {
        match reader.write().next().await {
            Some(Ok(message)) => {
                match message {
                    Message::Text(text) => serde_json::from_str(&text)
                        .map_err(|e| Error::OtherError(format!("Failed to parse message: {}", e))),
                    Message::Pong(_) => {
                        debug!("Received pong");
                        // 对于pong消息，我们创建一个虚拟的响应消息
                        Ok(WsResponseMessage {
                            event: Some(Operation::Pong),
                            code: Some("0".to_string()),
                            conn_id: Some(self.connection_id.clone()),
                            ..Default::default()
                        })
                    }
                    Message::Ping(_) => {
                        debug!("Received ping, sending pong");
                        // 对于ping消息，我们直接返回一个pong响应
                        writer.write().send(Message::Pong(Bytes::new())).await?;
                        Ok(WsResponseMessage {
                            event: Some(Operation::Pong),
                            code: Some("0".to_string()),
                            conn_id: Some(self.connection_id.clone()),
                            ..Default::default()
                        })
                    }
                    Message::Close(_) => {
                        warn!("WebSocket connection closed by server");
                        Err(Error::Other("WebSocket connection closed".into()))
                    }
                    _ => Err(Error::Other("Unsupported message type".into())),
                }
            }
            Some(Err(e)) => Err(Error::OtherError(format!("WebSocket error: {}", e))),
            None => Err(Error::Other("WebSocket connection closed".into())),
        }
    }

    /// 处理接收到的消息
    async fn handle_received_message(
        &self,
        channel_type: &WebSocketChannelType,
        message: WsResponseMessage,
    ) -> Result<()> {
        // 检查是否是认证响应
        if channel_type == &WebSocketChannelType::Private {
            if message.event == Some(Operation::Login) {
                if message.code == Some("0".to_string()) {
                    // 认证成功
                    self.auth_status.write().mark_authenticated();
                } else {
                    // 认证失败
                    self.auth_status.write().mark_failed();
                    return Err(Error::OtherError(format!(
                        "Authentication failed: {}",
                        message
                            .msg
                            .clone()
                            .unwrap_or_else(|| "Unknown error".to_string())
                    )));
                }
            } else if message.event == Some(Operation::Error) {
                // 处理错误消息
                error!(
                    "[{}]::Received error message: {}",
                    channel_type.as_str(),
                    message
                        .msg
                        .clone()
                        .unwrap_or_else(|| "Unknown error".to_string())
                );
                return Err(Error::OtherError(format!(
                    "Error from server: {}",
                    message.msg.unwrap_or_else(|| "Unknown error".to_string())
                )));
            }
        }

        // 将消息发送到相应的队列
        self.message_queue_manager
            .send_message(channel_type, message)
            .await?;
        Ok(())
    }

    /// 发送ping消息
    async fn send_ping(&self, sender: &Arc<RwLock<WsSink>>) -> Result<()> {
        sender
            .write()
            .send(Message::Ping(Bytes::new()))
            .await
            .map_err(|e| Error::OtherError(format!("Failed to send ping: {}", e)))
    }

    /// 订阅
    pub async fn subscribe(
        &mut self,
        args: &[WsArg],
        channel_type: &WebSocketChannelType,
    ) -> Result<()> {
        self.send_operation(&Operation::Subscribe, args, channel_type)
            .await
    }

    /// 取消订阅  
    pub async fn unsubscribe(
        &mut self,
        args: &[WsArg],
        channel_type: &WebSocketChannelType,
    ) -> Result<()> {
        self.send_operation(&Operation::Unsubscribe, args, channel_type)
            .await
    }

    /// 发送操作请求
    async fn send_operation(
        &mut self,
        operation: &Operation,
        args: &[WsArg],
        channel_type: &WebSocketChannelType,
    ) -> Result<()> {
        // 如果是私有通道且未认证，先进行认证
        if channel_type == &WebSocketChannelType::Private
            && !self.auth_status.read().is_authenticated
        {
            self.authenticate().await?;
        }

        let body = self.build_request_body(operation, args)?;
        debug!(
            "Sending {} operation to {} channel: {:?}",
            operation.as_str(),
            channel_type.as_str(),
            body
        );
        let connection = self.connection_manager.get_connection(channel_type)?;

        if let Some(sender) = &connection.sender {
            sender
                .write()
                .send(Message::Text(Utf8Bytes::from(serde_json::to_string(
                    &body,
                )?)))
                .await?;
            debug!(
                "Sent {} operation to {} channel",
                operation.as_str(),
                channel_type.as_str()
            );
            Ok(())
        } else {
            Err(Error::OtherError(format!(
                "{} connection not available",
                channel_type.as_str()
            )))
        }
    }

    /// 构建请求体
    fn build_request_body(
        &self,
        operation: &Operation,
        args: &[WsArg],
    ) -> Result<WsRequestMessage> {
        let mut client_body = OkexWsClientBodys::new();
        for arg in args {
            match arg {
                WsArg::Public(public_arg) => client_body.public_body.push(public_arg.as_value()?),
                WsArg::Private(private_arg) => {
                    client_body.private_body.push(private_arg.as_value()?);
                }
                WsArg::Business => {
                    return Err(Error::Other("Business arguments are not supported".into()));
                }
            }
        }

        let mut new_args = Vec::new();
        let mut validate_arg_category_count = 0;
        if !client_body.public_body.is_empty() {
            validate_arg_category_count += 1;
            new_args.extend(client_body.public_body);
        }
        if !client_body.private_body.is_empty() {
            validate_arg_category_count += 1;
            new_args.extend(client_body.private_body);
        }
        if !client_body.business_body.is_empty() {
            validate_arg_category_count += 1;
            new_args.extend(client_body.business_body);
        }
        if validate_arg_category_count > 1 {
            return Err(Error::Other("Too many argument categories provided".into()));
        }

        let body: WsRequestMessage = WsRequestMessage {
            id: None,
            op: operation.clone(),
            args: new_args,
        };
        Ok(body)
    }

    /// 构建参数
    fn build_args(&self, args: &[WsArg], operation: &Operation) -> Result<OkexWsClientBodys> {
        if operation == &Operation::Login && args.len() != 1 {
            return Err(Error::Other(
                "Login operation requires exactly one argument".into(),
            ));
        }

        let mut client_body = OkexWsClientBodys::new();
        let mut category_count = 0;

        for arg in args {
            match arg {
                WsArg::Public(public_arg) => {
                    if client_body.public_body.is_empty() {
                        category_count += 1;
                    }
                    client_body.public_body.push(public_arg.as_value()?);
                }
                WsArg::Private(private_arg) => {
                    // 对于登录操作，即使未认证也要添加参数
                    if operation == &Operation::Login || self.auth_status.read().is_authenticated {
                        if client_body.private_body.is_empty() {
                            category_count += 1;
                        }
                        client_body.private_body.push(private_arg.as_value()?);
                    }
                }
                WsArg::Business => {
                    return Err(Error::Other("Business arguments are not supported".into()));
                }
            }
        }

        if category_count > 1 {
            return Err(Error::Other(
                "Cannot mix different argument categories".into(),
            ));
        }

        Ok(client_body)
    }

    /// 获取WebSocket URL
    fn get_websocket_url(&self, channel_type: &WebSocketChannelType) -> Result<String> {
        let credentials = self
            .credentials
            .as_ref()
            .ok_or_else(|| Error::Other("Credentials not loaded".into()))?;

        if credentials.protocol != Some(Protocol::WSS) {
            return Err(Error::Other("Protocol is not WSS".into()));
        }

        let base_url = credentials
            .ws_urls
            .first()
            .ok_or_else(|| Error::Other("No WebSocket URL found in credentials".into()))?;

        Ok(format!("{}{}", base_url, channel_type.as_str()))
    }

    /// 生成签名
    fn generate_signature(
        &self,
        method: &Method,
        path: &str,
        body: &str,
        timestamp: f64,
        secret: &str,
    ) -> Result<String> {
        let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes())?;
        let message = format!("{}{}{}{}", timestamp, method.as_str(), path, body);
        mac.update(message.as_bytes());

        let result = mac.finalize();
        Ok(general_purpose::STANDARD.encode(result.into_bytes()))
    }

    /// 获取客户端状态
    pub fn get_status(&self) -> ClientStatus {
        ClientStatus {
            connection_id: self.connection_id.clone(),
            connection_health: self.connection_manager.health_check(),
            auth_status: self.auth_status.read().clone(),
            message_queue_state: self.get_message_queue_state(),
        }
    }

    /// 获取消息队列状态
    pub fn get_message_queue_state(&self) -> MessageQueueState {
        MessageQueueState {
            public_queue_size: self.message_queue_manager.public_sender.len(),
            private_queue_size: self.message_queue_manager.private_sender.len(),
            business_queue_size: self.message_queue_manager.business_sender.len(),
            error_queue_size: self.message_queue_manager.error_sender.len(),
        }
    }

    /// 关闭所有连接
    pub fn shutdown(&mut self) {
        info!("Shutting down OkexWebSocketClient");
        self.connection_manager.disconnect_all();
        self.auth_status.write().reset();
    }
}

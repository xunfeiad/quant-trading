pub mod binance;
pub mod channel_priority;
pub mod constant;
pub mod error;
pub mod okex;
pub mod okex_copy;
pub mod schema;

use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::protocol::Message};

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsReader = SplitStream<WsStream>;

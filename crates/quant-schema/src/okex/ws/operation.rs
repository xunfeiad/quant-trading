use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Default, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    #[default]
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "subscribe")]
    Subscribe,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
    #[serde(rename = "error")]
    Error,

    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "channel-conn-count")]
    ChannelConnCount,
}

impl Operation {
    pub fn as_str(&self) -> &str {
        match self {
            Operation::Login => "login",
            Operation::Subscribe => "subscribe",
            Operation::Unsubscribe => "unsubscribe",
            Operation::Error => "error",
            Operation::Ping => "ping",
            Operation::Pong => "pong",
            Operation::ChannelConnCount => "channel-conn-count",
        }
    }
    
}

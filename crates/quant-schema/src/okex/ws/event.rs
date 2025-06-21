use super::operation::Operation;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WsRequestMessage {
    pub id: Option<u64>,
    pub op: Operation,
    pub args: Vec<Value>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct WsResponseMessage {
    pub id: Option<u64>,
    pub event: Option<Operation>,
    pub arg: Option<Value>,
    pub code: Option<String>,
    pub msg: Option<String>,
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(rename = "curPage")]
    pub cur_page: Option<i32>,
    #[serde(rename = "last_page")]
    pub last_page: Option<bool>,
    pub data: Option<Value>,
    #[serde(rename = "connId")]
    pub conn_id: Option<String>,
}

impl WsResponseMessage {
    pub fn error(code: String, msg: String) -> Self {
        Self {
            id: None,
            event: Some(Operation::Error),
            arg: None,
            code: Some(code),
            msg: Some(msg),
            event_type: None,
            cur_page: None,
            last_page: None,
            data: None,
            conn_id: None,
        }
    }
}

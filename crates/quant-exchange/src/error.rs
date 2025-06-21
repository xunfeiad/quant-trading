use quant_schema::okex::ws::event::WsResponseMessage;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidLength(#[from] hmac::digest::InvalidLength),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Exchange not found in config: {0}")]
    ConfigError(&'static str),
    #[error("Parse error: {0}")]
    ParseError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Signature error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("{0}")]
    Other(&'static str),
    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error(transparent)]
    WSHTTPError(#[from] tokio_tungstenite::tungstenite::http::Error),
    #[error(transparent)]
    ConfigCratesError(#[from] quant_config::Error),

    #[error(transparent)]
    QuantSchemaError(#[from] quant_schema::error::Error),
    #[error(transparent)]
    FlumeTrySendError(#[from] flume::SendError<WsResponseMessage>),
    #[error(transparent)]
    TokioError(#[from] tokio::task::JoinError),
    #[error("Error: {0}")]
    OtherError(String),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Other(value)
    }
}

use std::io;

use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ParseFileError(#[from] io::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
    #[error("Configuration error: {0}")]
    ConfigError(&'static str),
}

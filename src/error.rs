// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BgpdError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Other error: {0}")]
    Other(String),
}

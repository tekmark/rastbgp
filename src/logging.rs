// src/logging.rs
use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

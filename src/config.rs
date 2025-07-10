// src/config.rs
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub local_asn: u16,
    pub bgp_id: String,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let cfg = toml::from_str(&contents)?;
        Ok(cfg)
    }
}

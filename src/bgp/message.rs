// src/bgp/message.rs

pub enum BgpMessage {
    Open,
    Update,
    Notification,
    Keepalive,
}

impl BgpMessage {
    pub fn encode(&self) -> Vec<u8> {
        // TODO: implement serialization
        vec![]
    }

    pub fn decode(_buf: &[u8]) -> Result<Self, String> {
        // TODO: implement parsing
        Ok(BgpMessage::Keepalive)
    }
}

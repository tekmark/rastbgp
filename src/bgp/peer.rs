// src/bgp/peer.rs

use tokio::net::TcpStream;

pub struct Peer {
    pub ip: String,
    pub asn: u16,
    pub stream: TcpStream,
}

impl Peer {
    pub async fn new(ip: String, asn: u16, stream: TcpStream) -> Self {
        Peer { ip, asn, stream }
    }

    pub async fn send_message(&mut self, _msg: super::message::BgpMessage) {
        // TODO: serialize and send message over TCP stream
    }

    pub async fn receive_message(&mut self) -> Option<super::message::BgpMessage> {
        // TODO: read from TCP stream and parse message
        None
    }
}

use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::io::{ReadHalf, WriteHalf};

use crate::bgp::fsm::{Fsm, BgpEvent};

pub struct Peer {
    pub remote_ip: IpAddr,
    // RFC 4893, 4 bytes for AS numbers
    pub remote_asn: u32,
    pub fsm: Fsm,
    pub reader: Option<ReadHalf<TcpStream>>,
    pub writer: Option<WriteHalf<TcpStream>>,
}

impl Peer {
    pub fn new(remote_ip: IpAddr, remote_asn: u32) -> Self {
        Self {
            remote_ip,
            remote_asn,
            fsm: Fsm::new(),
            reader: None,
            writer: None,
        }
    }

    pub fn from_stream(ip: IpAddr, stream: TcpStream) -> Self {
        let (reader, writer) = tokio::io::split(stream);
        let mut peer = Self::new(ip, 64512); // Replace with real config if needed
        peer.reader = Some(reader);
        peer.writer = Some(writer);
        peer
    }

    pub fn on_event(&mut self, event: BgpEvent) {
        self.fsm.handle_event(event);
    }

    pub fn state(&self) -> crate::bgp::fsm::BgpState {
        self.fsm.state()
    }
}

use crate::bgp::fsm::{BgpEvent, BgpState};
use crate::bgp::message::BgpMessage;
use crate::bgp::peer::Peer;

use tokio::{net::TcpStream, time};
use bytes::{BytesMut, BufMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;

pub struct PeerRunner {
    pub peer: Peer,
}

impl PeerRunner {
    pub fn new(peer: Peer) -> Self {
        Self { peer }
    }

    pub async fn run(mut self) {
        self.peer.on_event(BgpEvent::ManualStart);

        match TcpStream::connect((self.peer.remote_ip, 179)).await {
            Ok(stream) => {
                let (reader, writer) = tokio::io::split(stream);
                self.peer.reader = Some(reader);
                self.peer.writer = Some(writer);
                self.peer.on_event(BgpEvent::TcpConnectionSuccess);
            }
            Err(e) => {
                eprintln!("Failed to connect to peer: {}", e);
                self.peer.on_event(BgpEvent::TcpConnectionFail);
                return;
            }
        }

        // Send BGP OPEN
        if let Some(writer) = &mut self.peer.writer {
            let open = BgpMessage::Open.encode();
            if let Err(e) = writer.write_all(&open).await {
                eprintln!("Failed to send OPEN: {}", e);
                return;
            }
        }

        let mut buf = BytesMut::with_capacity(4096);
        let mut hold_timer = time::sleep(Duration::from_secs(90));
        tokio::pin!(hold_timer);

        loop {
            tokio::select! {
                result = self.peer.reader.as_mut().unwrap().read_buf(&mut buf) => {
                    match result {
                        Ok(0) => {
                            println!("Peer disconnected.");
                            break;
                        }
                        Ok(_) => {
                            if let Ok(msg) = BgpMessage::parse(buf.split()) {
                                match msg {
                                    BgpMessage::Open => self.peer.on_event(BgpEvent::BgpOpenReceived),
                                    BgpMessage::Keepalive => self.peer.on_event(BgpEvent::KeepaliveReceived),
                                    BgpMessage::Notification => self.peer.on_event(BgpEvent::NotificationReceived),
                                    BgpMessage::Update => {
                                        // TODO: Handle UPDATE messages
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break;
                        }
                    }
                }
                _ = &mut hold_timer => {
                    self.peer.on_event(BgpEvent::HoldTimerExpired);
                    break;
                }
            }
        }
    }
}

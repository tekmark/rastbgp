use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use crate::bgp::{Peer, PeerRunner};

pub struct PeerManager;

impl PeerManager {
    pub async fn start_listener(addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        tracing::info!("Listening for BGP connections on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    tracing::info!("Accepted connection from {}", peer_addr);

                    let peer = Peer::from_stream(peer_addr.ip(), stream);
                    let runner = PeerRunner::new(peer);

                    tokio::spawn(async move {
                        runner.run().await;
                    });
                }
                Err(e) => {
                    tracing::error!("Listener error: {}", e);
                }
            }
        }
    }
}

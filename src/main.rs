use tonic::transport::Server;
use rastbgp::grpc::proto::bgp::bgp_service_server::BgpServiceServer;
use rastbgp::grpc::service::MyBgpService;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    let bgp_service = MyBgpService::default();

    println!("BGP gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(BgpServiceServer::new(bgp_service))
        .serve(addr)
        .await?;

    Ok(())
}

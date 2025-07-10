// src/grpc/service.rs
use tonic::{Request, Response, Status};
use crate::grpc::proto::bgp::bgp_service_server::{BgpService, BgpServiceServer};
use crate::grpc::proto::bgp::{StatusRequest, StatusResponse};

#[derive(Default)]
pub struct MyBgpService {}

#[tonic::async_trait]
impl BgpService for MyBgpService {
    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let reply = StatusResponse {
            state: "Idle".to_string(),
            peers: 0,
        };
        Ok(Response::new(reply))
    }
}

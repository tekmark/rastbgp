use warp::Filter;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::net::SocketAddr;
use tokio::task;

pub struct Metrics {
    handle: PrometheusHandle,
}

impl Metrics {
    pub fn new() -> Self {
        let builder = PrometheusBuilder::new();
        let handle = builder.install_recorder().expect("Failed to install Prometheus recorder");
        Metrics { handle }
    }

    pub fn handle(&self) -> &PrometheusHandle {
        &self.handle
    }

    pub async fn serve(&self, addr: SocketAddr) {
        let handle = self.handle.clone();
        task::spawn(async move {
            warp::serve(
                warp::path("metrics").map(move || {
                    let body = handle.render();
                    warp::reply::with_header(body, "Content-Type", "text/plain; version=0.0.4")
                }),
            )
            .run(addr)
            .await;
        });
    }
}

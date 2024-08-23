use main::mnt_server::{Mnt, MntServer};
// use main::signal_service_client::SignalServiceClient;
use main::{DriveInfo, MntRequest,TotalSpaceInfo};

use tokio::join;
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status};

pub mod main {
    tonic::include_proto!("fetch_is_mnt");
}

pub async fn start() {
    _ = join!(server())
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    info!("GRPC Server listening on {}", addr);

    Server::builder()
        .add_service(MntServer::new(MntRealization::default()))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct MntRealization {}

#[async_trait]
impl Mnt for MntRealization {
    async fn drive(
        &self,
        request: Request<MntRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<DriveInfo>, Status> {
        let reply = DriveInfo {
            mount: request.into_inner().mntpoint,
            name: "not yet".to_owned(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn total_space(
        &self,
        request: tonic::Request<MntRequest>,
    ) -> std::result::Result<tonic::Response<TotalSpaceInfo>, Status> {
        Ok(Response::new(TotalSpaceInfo {
            mount: "bebra".to_owned(),
            total_space: 12,
        }))
    }
}

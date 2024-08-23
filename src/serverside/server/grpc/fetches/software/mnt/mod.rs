use tonic::{async_trait, Request, Response, Status};
use sysinfo::{Disk, Disks};

tonic::include_proto!("fetch_is_mnt");

#[derive(Debug, Default)]
pub struct MntRealization {}

#[async_trait]
impl mnt_server::Mnt for MntRealization {
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
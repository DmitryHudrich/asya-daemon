use fetches::software;
use tokio::join;
use tonic::transport::Server;

pub mod fetches;

pub async fn start() {
    _ = join!(server())
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    info!("GRPC Server listening on {}", addr);

    Server::builder()
        .add_service(software::mnt::mnt_server::MntServer::new(software::mnt::MntRealization::default()))
        .add_service(software::sys::sys_server::SysServer::new(software::sys::SysRealisation::default()))
        .serve(addr)
        .await?;

    Ok(())
}

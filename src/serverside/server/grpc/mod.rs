use tokio::join;
use tonic::transport::Server;

pub mod fetch;

pub async fn start() {
    _ = join!(server())
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    info!("GRPC Server listening on {}", addr);

    Server::builder()
        .add_service(fetch::mnt_server::MntServer::new(fetch::MntRealization::default()))
        .serve(addr)
        .await?;

    Ok(())
}

use services::fetches;
use tokio::join;
use tonic::transport::Server;

use crate::configuration;

pub mod services;

pub async fn start() {
    _ = join!(server())
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", configuration::get().net().grpc_port()).parse().unwrap();

    info!("GRPC Server listening on {}", addr);

    Server::builder()
        .add_service(fetches::software::mnt::mnt_server::MntServer::new(
            fetches::software::mnt::MntRealization::default(),
        ))
        .add_service(fetches::software::sys::sys_server::SysServer::new(
            fetches::software::sys::SysRealisation::default(),
        ))
        .add_service(fetches::hardware::cpu::cpu_server::CpuServer::new(
            fetches::hardware::cpu::CpuRealisation::default(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}

use sysinfo::System;
use tonic::{async_trait, Request, Response, Status};

tonic::include_proto!("fetch_is_sys");

#[derive(Debug, Default)]
pub struct SysRealisation {}

#[async_trait]
impl sys_server::Sys for SysRealisation {
    async fn name(&self, _: Request<SysRequest>) -> Result<Response<NameInfo>, Status> {
        Ok(Response::new(NameInfo {
            name: System::name().unwrap_or_default(),
        }))
    }

    async fn kernel_version(
        &self,
        _: Request<SysRequest>,
    ) -> Result<Response<KernelVersionInfo>, Status> {
        Ok(Response::new(KernelVersionInfo {
            kernel_version: System::kernel_version().unwrap_or_default(),
        }))
    }

    async fn os_version(&self, _: Request<SysRequest>) -> Result<Response<OsVersionInfo>, Status> {
        Ok(Response::new(OsVersionInfo {
            os_version: System::os_version().unwrap_or_default(),
        }))
    }

    async fn long_os_version(
        &self,
        _: Request<SysRequest>,
    ) -> Result<Response<LongOsVersionInfo>, Status> {
        Ok(Response::new(LongOsVersionInfo {
            long_os_version: System::long_os_version().unwrap_or_default(),
        }))
    }

    async fn host(&self, _: Request<SysRequest>) -> Result<Response<HostInfo>, Status> {
        Ok(Response::new(HostInfo {
            host: System::host_name().unwrap_or_default(),
        }))
    }

    async fn uptime_seconds(
        &self,
        _: Request<SysRequest>,
    ) -> Result<Response<UptimeSecondsInfo>, Status> {
        Ok(Response::new(UptimeSecondsInfo {
            uptime_seconds: System::uptime(),
        }))
    }

    async fn distro_id(&self, _: Request<SysRequest>) -> Result<Response<DistroIdInfo>, Status> {
        Ok(Response::new(DistroIdInfo {
            distro_id: System::distribution_id(),
        }))
    }
}

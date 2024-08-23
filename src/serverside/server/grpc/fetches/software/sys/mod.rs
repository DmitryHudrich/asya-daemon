use tonic::{async_trait, Request, Response, Status};
use sysinfo::System;


tonic::include_proto!("fetch_is_sys");


#[derive(Debug, Default)]
pub struct  SysRealisation {}


#[async_trait]
impl sys_server::Sys for SysRealisation {
    async fn name(&self, _: Request<SysRequest>) -> Result<Response<NameInfo>, Status> {
        let res = NameInfo {
            name: System::name().unwrap_or_default(),
        };
        Ok(Response::new(res))
    }

    async fn kernel_version(&self, _: Request<SysRequest>) -> Result<Response<KernelVersionInfo>, Status> {
        let res = KernelVersionInfo {
            kernel_version: System::kernel_version().unwrap_or_default(),
        };
        Ok(Response::new(res))
    }

    async fn os_version(&self, _: Request<SysRequest>) -> Result<Response<OsVersionInfo>, Status> {
        let res = OsVersionInfo {
            os_version: System::os_version().unwrap_or_default()
        };
        Ok(Response::new(res))
    }

    async fn long_os_version(&self, _: Request<SysRequest>) -> Result<Response<LongOsVersionInfo>, Status> {
        let res = LongOsVersionInfo {
            long_os_version: System::long_os_version().unwrap_or_default()
        };
        Ok(Response::new(res))
    }

    async fn host(&self, _: Request<SysRequest>) -> Result<Response<HostInfo>, Status> {
        let res = HostInfo {
            host: System::host_name().unwrap_or_default()
        };
        Ok(Response::new(res))
    }

    async fn uptime_seconds(&self, _: Request<SysRequest>) -> Result<Response<UptimeSecondsInfo>, Status> {
        let res = UptimeSecondsInfo {
            uptime_seconds: System::uptime()
        };
        Ok(Response::new(res))
    }

    async fn distro_id(&self, _:Request<SysRequest>) -> Result<Response<DistroIdInfo>, Status> {
        let res = DistroIdInfo {
            distro_id: System::distribution_id()
        };
        Ok(Response::new(res))
    }
}

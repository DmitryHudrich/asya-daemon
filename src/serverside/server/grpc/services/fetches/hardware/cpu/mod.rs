use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tonic::{async_trait, Request, Response, Status};

tonic::include_proto!("fetch_is_cpu");

#[derive(Default, Debug)]
pub struct CpuRealisation {}

#[async_trait]
impl cpu_server::Cpu for CpuRealisation {
    async fn global_usage(
        &self,
        _: Request<CpuRequest>,
    ) -> Result<Response<GlobalUsageInfo>, Status> {
        Ok(Response::new(GlobalUsageInfo {
            global_usage: system(|sys| sys.global_cpu_usage() as u32),
        }))
    }

    async fn brand(&self, _: Request<CpuRequest>) -> Result<Response<BrandInfo>, Status> {
        Ok(Response::new(BrandInfo {
            brand: system(|sys| sys.cpus()[0].brand().to_string()),
        }))
    }

    async fn core_count(&self, _: Request<CpuRequest>) -> Result<Response<CoreCountInfo>, Status> {
        Ok(Response::new(CoreCountInfo {
            core_count: system(|sys| sys.cpus().len()) as u32,
        }))
    }

    async fn vendor(&self, _: Request<CpuRequest>) -> Result<Response<VendorInfo>, Status> {
        Ok(Response::new(VendorInfo {
            vendor: system(|sys| sys.cpus()[0].vendor_id().to_string()),
        }))
    }

    async fn name(&self, _: Request<CpuRequest>) -> Result<Response<NameInfo>, Status> {
        Ok(Response::new(NameInfo {
            name: system(|sys| sys.cpus()[0].name().to_string()),
        }))
    }

    async fn frequency(&self, _: Request<CpuRequest>) -> Result<Response<FrequencyInfo>, Status> {
        Ok(Response::new(FrequencyInfo {
            frequency: system(|sys| sys.cpus()[0].frequency()),
        }))
    }
}

fn system<T, F>(f: T) -> F
where
    T: FnOnce(&System) -> F,
{
    f(&System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    ))
}


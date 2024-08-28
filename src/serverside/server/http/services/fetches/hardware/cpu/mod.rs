use sysinfo::{CpuRefreshKind, RefreshKind, System};

tonic::include_proto!("fetches");

use crate::server::http::CpuFetch as HttpCpuFetch;

fn get_global_usage() -> Option<u64> {
    Some(system(|sys| sys.global_cpu_usage()).unwrap() as u64)
}

fn get_brand() -> Option<String> {
    system(|sys| sys.cpus()[0].brand().to_string())
}

fn get_core_count() -> Option<u64> {
    Some(system(|sys| sys.cpus().len()).unwrap() as u64)
}

fn get_vendor() -> Option<String> {
    system(|sys| sys.cpus()[0].vendor_id().to_string())
}

fn get_name() -> Option<String> {
    system(|sys| sys.cpus()[0].name().to_string())
}

fn get_frequency() -> Option<u64> {
    system(|sys| sys.cpus()[0].frequency())
}


fn system<T, F>(f: T) -> Option<F>
where
    T: FnOnce(&System) -> F,
{
    Some(f(&System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    )))
}

pub fn get_cpu_fetch() -> Option<HttpCpuFetch> {
  Some(HttpCpuFetch {
    global_usage: get_global_usage(),
    brand: get_brand(),
    core_count: get_core_count(),
    vendor: get_vendor(),
    name: get_name(),
    frequency: get_frequency()
  })
}
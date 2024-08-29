use sysinfo::System;

use crate::fetch_dto::SysFetch;

fn get_name() -> Option<String> {
    Some(System::name().unwrap_or_default())
}

fn get_kernel_version() -> Option<String> {
    Some(System::kernel_version().unwrap_or_default())
}

fn get_os_version() -> Option<String> {
    Some(System::os_version().unwrap_or_default())
}

fn get_long_os_version() -> Option<String> {
    Some(System::long_os_version().unwrap_or_default())
}

fn get_host() -> Option<String> {
    Some(System::host_name().unwrap_or_default())
}

fn get_uptime_seconds() -> Option<u64> {
    Some(System::uptime())
}

fn get_distro_id() -> Option<String> {
    Some(System::distribution_id())
}

pub fn get_sys_fetch() -> Option<SysFetch> {
  Some(SysFetch {
    name: get_name(),
    kernel_version: get_kernel_version(),
    os_version: get_os_version(),
    long_os_version: get_long_os_version(),
    host: get_host(),
    uptime_seconds: get_uptime_seconds(),
    distro_id: get_distro_id()
  })
}

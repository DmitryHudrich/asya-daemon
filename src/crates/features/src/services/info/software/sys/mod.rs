use sysinfo::System;


pub fn get_name(_: String) -> Option<String> {
    Some(System::name().unwrap_or_default())
}

pub fn get_kernel_version(_: String) -> Option<String> {
    Some(System::kernel_version().unwrap_or_default())
}

pub fn get_os_version(_: String) -> Option<String> {
    Some(System::os_version().unwrap_or_default())
}

pub fn get_long_os_version(_: String) -> Option<String> {
    Some(System::long_os_version().unwrap_or_default())
}

pub fn get_host(_: String) -> Option<String> {
    Some(System::host_name().unwrap_or_default())
}

pub fn get_uptime_seconds(_: String) -> Option<u64> {
    Some(System::uptime())
}

pub fn get_distro_id(_: String) -> Option<String> {
    Some(System::distribution_id())
}

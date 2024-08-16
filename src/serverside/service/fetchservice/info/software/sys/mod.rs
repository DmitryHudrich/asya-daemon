use serde_json::json;
use sysinfo::System;

type Json = serde_json::Value;

pub fn get_name(value: String) -> Json {
    json!(System::name().unwrap_or_default())
}

pub fn get_kernel_version(value: String) -> Json {
    json!(System::kernel_version().unwrap_or_default())
}

pub fn get_os_version(value: String) -> Json {
    json!(System::os_version().unwrap_or_default())
}

pub fn get_long_os_version(value: String) -> Json {
    json!(System::long_os_version().unwrap_or_default())
}

pub fn get_host(value: String) -> Json {
    json!(System::host_name().unwrap_or_default())
}

pub fn get_uptime_seconds(value: String) -> Json {
    json!(System::uptime())
}

pub fn get_distro_id(value: String) -> Json {
    json!(System::distribution_id())
}

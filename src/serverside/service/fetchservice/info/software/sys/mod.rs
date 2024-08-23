use serde_json::{json, Value};
use sysinfo::System;


pub fn get_name(_: String) -> Value {
    json!(System::name().unwrap_or_default())
}

pub fn get_kernel_version(_: String) -> Value {
    json!(System::kernel_version().unwrap_or_default())
}

pub fn get_os_version(_: String) -> Value {
    json!(System::os_version().unwrap_or_default())
}

pub fn get_long_os_version(_: String) -> Value {
    json!(System::long_os_version().unwrap_or_default())
}

pub fn get_host(_: String) -> Value {
    json!(System::host_name().unwrap_or_default())
}

pub fn get_uptime_seconds(_: String) -> Value {
    json!(System::uptime())
}

pub fn get_distro_id(_: String) -> Value {
    json!(System::distribution_id())
}

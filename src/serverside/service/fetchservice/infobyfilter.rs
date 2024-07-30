use std::collections::HashMap;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub fn new(params: HashMap<String, String>) -> serde_json::Value {
    let mut res = HashMap::new();
    for (key, value) in params {
        if let Some(field) = match_param(key.as_str(), value) {
            res.insert(key, field);
        }
    }
    serde_json::to_value(res).unwrap()
}

// TODO: Return u64 (or other type) instead String if needed.
fn match_param(key: &str, value: String) -> Option<String> {
    if value != "1" {
        return None;
    }
    // Prefixes list for field names:
    // s - system (basic system info e.g. name or version)
    // m - ram
    // d - drive
    // i - info
    //
    // for example:
    //  "siname" means system_info_name
    //  "sikernel_version" means system_info_kernel_version
    match key {
        "siname" => System::name(),
        "sikernel_version" => System::kernel_version(),
        "sios_version" => System::os_version(),
        "sihostname" => System::host_name(),
        "mitotal" => {
            let mut system = System::new_with_specifics(
                RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
            );
            system.refresh_memory();
            Some(system.total_memory().to_string())
        },
        "miused" => {
            let mut system = System::new_with_specifics(
                RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
            );
            system.refresh_memory();
            Some(system.used_memory().to_string())
        },
        "miswap_total" => {
            let mut system = System::new_with_specifics(
                RefreshKind::new().with_memory(MemoryRefreshKind::new().with_swap()),
            );
            system.refresh_memory();
            Some(system.total_swap().to_string())
        },
        "miswap_used" => {
            let mut system = System::new_with_specifics(
                RefreshKind::new().with_memory(MemoryRefreshKind::new().with_swap()),
            );
            system.refresh_memory();
            Some(system.used_swap().to_string())
        },
        _ => None,
    }
}

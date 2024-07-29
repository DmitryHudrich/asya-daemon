use serde::{Deserialize, Serialize};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Serialize, Deserialize, Default)]
pub struct BasicInfo {
    name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    hostname: Option<String>,
}

impl BasicInfo {
    pub fn new() -> BasicInfo {
        BasicInfo {
            name: System::name(),
            kernel_version: System::kernel_version(),
            os_version: System::os_version(),
            hostname: System::host_name(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct MemoryInfo {
    total_ram: u64,
    used_ram: u64,
    total_swap: u64,
    used_swap: u64,
}

impl MemoryInfo {
    pub fn new() -> MemoryInfo {
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        sys.refresh_memory();
        MemoryInfo {
            total_ram: sys.total_memory(),
            used_ram: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
        }
    }
}

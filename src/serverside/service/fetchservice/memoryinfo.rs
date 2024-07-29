use serde::Deserialize;
use serde::Serialize;
use sysinfo::MemoryRefreshKind;
use sysinfo::RefreshKind;
use sysinfo::System;

#[derive(Serialize, Deserialize, Default)]
pub struct MemoryInfo {
    pub total_ram: u64,
    pub used_ram: u64,
    pub total_swap: u64,
    pub used_swap: u64,
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

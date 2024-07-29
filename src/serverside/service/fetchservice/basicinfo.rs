use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Serialize, Deserialize, Default)]
pub struct BasicInfo {
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub hostname: Option<String>,
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


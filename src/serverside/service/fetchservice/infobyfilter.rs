use multimap::MultiMap;
use serde::Serialize;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub fn new(params: MultiMap<String, String>) -> serde_json::Value {
    let mut res = MultiMap::new();
    for (key, value) in &params {
        for element in value {
            if let Some(field) = match_param(key.as_str(), element.as_str()) {
                debug!("Query parsing | \n\tKey: {key}\n\tValue:{element}");
                res.insert(key, field);
            }
        }
    }
    serde_json::to_value(res).unwrap()
}

#[derive(Serialize)]
pub enum MatchParam {
    StringRes (String),
    U32Res (u64)
}

// TODO: Return u64 (or other type) instead String if needed.
fn match_param(key: &str, value: &str) -> Option<MatchParam> {
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
    let system = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram().with_swap())
    );

    match key {
        "siname" => Some(MatchParam::StringRes(System::name().unwrap())),
        "sikernel_version" => Some(MatchParam::U32Res(System::kernel_version().unwrap().trim().parse::<u64>().unwrap())),
        "sios_version" => Some(MatchParam::U32Res(System::os_version().unwrap().trim().parse::<u64>().unwrap())),
        "sihostname" => Some(MatchParam::StringRes(System::host_name().unwrap())),
        "mitotal" => Some(MatchParam::U32Res(system.total_memory())),
        "miused" => Some(MatchParam::U32Res(system.used_memory())),
        "miswap_total" => Some(MatchParam::U32Res(system.total_swap())),
        "miswap_used" => Some(MatchParam::U32Res(system.used_swap())),
        _ => None
    }


    // match key {
    //     "siname" => Some(MatchParam::StringRes(System::name().unwrap())),
    //     "sikernel_version" => Some(MatchParam::U32Res(System::kernel_version().unwrap().trim().parse::<u64>().expect("Hhbd"))),
    //     "sios_version" => Some(MatchParam::U32Res(System::os_version().unwrap().trim().parse::<u64>().unwrap())),
    //     "sihostname" => Some(MatchParam::StringRes(System::host_name().unwrap())),
    //     "mitotal" => {
    //         let mut system = System::new_with_specifics(
    //             RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
    //         );
    //         system.refresh_memory();
    //         Some(MatchParam::U32Res(system.total_memory()))
    //     },
    //     "miused" => {
    //         let mut system = System::new_with_specifics(
    //             RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
    //         );
    //         system.refresh_memory();
    //         Some(MatchParam::U32Res(system.used_memory()))
    //     },
    //     "miswap_total" => {
    //         let mut system = System::new_with_specifics(
    //             RefreshKind::new().with_memory(MemoryRefreshKind::new().with_swap()),
    //         );
    //         system.refresh_memory();
    //         Some(MatchParam::U32Res(system.total_swap()))
    //     },
    //     "miswap_used" => {
    //         let mut system = System::new_with_specifics(
    //             RefreshKind::new().with_memory(MemoryRefreshKind::new().with_swap()),
    //         );
    //         system.refresh_memory();
    //         Some(MatchParam::U32Res(system.used_swap()))
    //     },
    //     _ => None,
    // }
}

use sysinfo::{Disk, Disks};

use crate::fetch_dto::DrvFetch;

fn get_total_space(value: &String) -> Option<u64> {
    identify_disk(&value, |di| di.total_space())
}

fn get_available_space(value: &String) -> Option<u64> {
    identify_disk(&value, |di| di.available_space())
}

fn get_used_space(value: &String) -> Option<u64> {
    identify_disk(&value, |di| di.total_space() - di.available_space())
}

fn get_kind(value: &String) -> Option<String> {
    identify_disk(&value, |di| di.kind().to_string())
}

fn get_file_system(value: &String) -> Option<String> {
    identify_disk(&value, |di| di.file_system().to_str().unwrap().to_string())
}

fn get_is_removable(value: &String) -> Option<bool> {
    identify_disk(&value, |di| di.is_removable())
}

fn get_mount(value: &String) -> Option<String> {
    identify_disk(&value, |di| di.mount_point().to_str().unwrap().to_string())
}

fn identify_disk<F, T>(value: &str, f: F) -> Option<T>
where
    F: Fn(&Disk) -> T
{
  Disks::new_with_refreshed_list()
    .into_iter()
    .find(|disk| disk.mount_point().to_str().unwrap_or_default() == value)
    .map(f)
}

pub fn get_drv_fetch(value: &String) -> Option<DrvFetch> {
  Some(DrvFetch {
    total_space: get_total_space(value),
    available_space: get_available_space(value),
    used_space: get_used_space(value),
    kind: get_kind(value),
    file_system: get_file_system(value),
    is_removable: get_is_removable(value),
    mount: get_mount(value)
  })
}

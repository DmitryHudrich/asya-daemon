use sysinfo::{Disk, Disks};

use crate::fetches::fetch_dto::MntFetch;

fn get_drive(value: &String) -> Option<String> {
    identify_disk(&value, |di| di.name().to_str().unwrap().to_owned())
}

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


fn identify_disk<F, T>(value: &str, f: F) -> Option<T>
where
    F: Fn(&Disk) -> T
{
  Disks::new_with_refreshed_list()
    .into_iter()
    .find(|disk| disk.mount_point().to_str().unwrap_or_default() == value)
    .map(f)
}

pub fn get_mnt_fetch(value: &String) -> Option<MntFetch> {
  Some(MntFetch {
    drive: get_drive(value),
    total_space: get_total_space(value),
    avaible_space: get_available_space(value),
    used_space: get_used_space(value),
    kind: get_kind(value),
    file_system: get_file_system(value),
    is_removable: get_is_removable(value)
  })
}

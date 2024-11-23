use std::path::PathBuf;
use sysinfo::{Disks, Disk};


pub fn get_total_space(value: String) -> Option<u64> {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .and_then(|d| Some(d.total_space()))
}

pub fn get_available_space(value: String) -> Option<u64> {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .and_then(|d| Some(d.available_space()))
}

pub fn get_used_space(value: String) -> Option<u64> {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .and_then(|d| Some(d.total_space() - d.available_space()))
}

pub fn get_kind(value: String) -> Option<String> {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .and_then(|d| Some(d.kind().to_string()))
}

pub fn get_file_system(value: String) -> Option<String> {
    let disks = Disks::new_with_refreshed_list();
    identify_disk(&value, &disks)
        .map(|d| d.file_system().to_str().unwrap_or_default().to_string())
}

pub fn get_is_removable(value: String) -> Option<bool> {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .and_then(|d| Some(d.is_removable()))
}

pub fn get_mount(value: &String) -> Option<PathBuf> {
    let disks = Disks::new_with_refreshed_list();
    identify_disk(value, &disks)
        .map(|d| d.mount_point().to_path_buf())
}

fn identify_disk<'a>(value: &str, disks: &'a Disks) -> Option<&'a Disk> {
    disks.into_iter()
        .find(|disk| disk.mount_point().to_str().unwrap() == value)
}
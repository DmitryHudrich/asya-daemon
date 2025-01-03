use sysinfo::{Disk, Disks};

pub fn get_drive(value: String) -> Option<String> {
    let disks = Disks::new_with_refreshed_list();
    identify_disk(&value, &disks).map(|d| d.name().to_str().unwrap_or_default().to_string())
}

pub fn get_total_space(value: String) -> Option<u64> {
    identify_disk(&value, &Disks::new_with_refreshed_list()).map(|d| d.total_space())
}

pub fn get_available_space(value: String) -> Option<u64> {
    identify_disk(&value, &Disks::new_with_refreshed_list()).map(|d| d.available_space())
}

pub fn get_used_space(value: String) -> Option<u64> {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map(|d| d.total_space() - d.available_space())
}

pub fn get_kind(value: String) -> Option<String> {
    identify_disk(&value, &Disks::new_with_refreshed_list()).map(|d| d.kind().to_string())
}

pub fn get_file_system(value: String) -> Option<String> {
    let disks = Disks::new_with_refreshed_list();
    identify_disk(&value, &disks).map(|d| d.file_system().to_str().unwrap_or_default().to_string())
}

pub fn get_is_removable(value: String) -> Option<bool> {
    identify_disk(&value, &Disks::new_with_refreshed_list()).map(|d| d.is_removable())
}

fn identify_disk<'a>(value: &str, disks: &'a Disks) -> Option<&'a Disk> {
    disks
        .into_iter()
        .find(|&disk| disk.mount_point().to_str().unwrap() == value)
}

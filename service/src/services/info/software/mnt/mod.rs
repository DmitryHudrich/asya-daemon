use serde_json::{json, Value};
use sysinfo::Disks;


pub fn get_drive(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.name().to_str().unwrap()))
}

pub fn get_total_space(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.total_space()))
}

pub fn get_available_space(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.available_space()))
}

pub fn get_used_space(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.total_space() - d.available_space()))
}

pub fn get_kind(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.kind().to_string()))
}

pub fn get_file_system(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.file_system().to_str().unwrap_or_default()))
}

pub fn get_is_removable(value: String) -> Value {
    identify_disk(&value, &Disks::new_with_refreshed_list())
        .map_or(Value::Null, |d| json!(d.is_removable()))
}

fn identify_disk<'a>(value: &str, disks: &'a Disks) -> std::option::Option<&'a sysinfo::Disk> {
    disks.into_iter()
        .find(|&disk| disk.mount_point().to_str().unwrap() == value)
}
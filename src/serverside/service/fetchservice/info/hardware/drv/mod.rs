use serde_json::{json, Value};
use sysinfo::{Disks, Disk};


// А нахуя?
// Запрос: получить имя диска /dev/sda1
// Ответ: /dev/sda1
//
// pub fn get_name(value: String) -> Value {
//     let disks = Disks::new_with_refreshed_list();
//     let disk = identify_disk(&value, &disks);
//     match disk {
//         Some(disk) => json!(disk.name().to_str().unwrap()),
//         None => Value::Null,
//     }
// }

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

pub fn get_mount(value: String) -> Value {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.mount_point()),
        None => Value::Null,
    }
}

fn identify_disk<'a>(value: &str, disks: &'a Disks) -> Option<&'a Disk> {
    disks.into_iter()
        .find(|&disk| disk.mount_point().to_str().unwrap() == value)
}
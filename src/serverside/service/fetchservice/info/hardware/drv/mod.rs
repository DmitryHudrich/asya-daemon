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
    identify_disk(&value, |di| di.total_space())
}

pub fn get_available_space(value: String) -> Value {
    identify_disk(&value, |di| di.available_space())
}


pub fn get_used_space(value: String) -> Value {
    identify_disk(&value, |di| di.total_space() - di.available_space())
}

pub fn get_kind(value: String) -> Value {
    identify_disk(&value, |di| di.kind().to_string())
}

pub fn get_file_system(value: String) -> Value {
    identify_disk(&value, |di| di.file_system().to_owned())
}

pub fn get_is_removable(value: String) -> Value {
    identify_disk(&value, |di| di.is_removable())
}

pub fn get_mount(value: String) -> Value {
    identify_disk(&value, |di| di.mount_point().to_owned())
}

fn identify_disk<T, F>(value: &str, f: F) -> Value
where
    F: Fn(&Disk) -> T,
    T: serde::Serialize,
{
    Disks::new_with_refreshed_list().into_iter()
        .find(|disk| disk.mount_point().to_str().unwrap_or_default() == value)
        .map_or(Value::Null, |d| json!(f(&d)))
}

use serde_json::json;
use sysinfo::Disks;

type Json = serde_json::Value;

// А нахуя?
// Запрос: получить имя диска /dev/sda1
// Ответ: /dev/sda1
//
// pub fn get_name(value: String) -> Json {
//     let disks = Disks::new_with_refreshed_list();
//     let disk = identify_disk(&value, &disks);
//     match disk {
//         Some(disk) => json!(disk.name().to_str().unwrap()),
//         None => Json::Null,
//     }
// }

pub fn get_total_space(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.total_space()),
        None => Json::Null,
    }
}

pub fn get_available_space(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.available_space()),
        None => Json::Null,
    }
}

pub fn get_used_space(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.total_space() - disk.available_space()),
        None => Json::Null,
    }
}

pub fn get_kind(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.kind().to_string()),
        None => Json::Null,
    }
}

pub fn get_file_system(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.file_system().to_str().unwrap_or_default()),
        None => Json::Null,
    }
}

pub fn get_is_removable(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.is_removable()),
        None => Json::Null,
    }
}

pub fn get_mount(value: String) -> Json {
    match identify_disk(&value, &Disks::new_with_refreshed_list()) {
        Some(disk) => json!(disk.mount_point()),
        None => Json::Null,
    }
}

fn identify_disk<'a>(
    value: &str,
    disks: &'a Disks
    ) -> std::option::Option<&'a sysinfo::Disk> {
    let disk = disks
        .into_iter()
        .find(|disk| disk.name().to_str().unwrap() == value);
    disk
}

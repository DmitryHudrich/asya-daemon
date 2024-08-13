use serde_json::json;
use sysinfo::Disks;

pub fn get_diskname_by_mountpoint(value: String) -> serde_json::Value {
    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .into_iter()
        .find(|disk| disk.mount_point().to_str().unwrap() == value.as_str());
    
    match disk {
        Some(disk) => json!(disk.name().to_str().unwrap()),
        None => serde_json::Value::Null,
    }
}

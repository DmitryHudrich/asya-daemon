use std::collections::HashMap;

use multimap::MultiMap;
use serde_json::{json, Map, Number, Value};
use sysinfo::{Disks, System};

pub fn new(params: MultiMap<String, String>) -> serde_json::Value {
    let mut res = HashMap::new();
    for (key, value) in &params {
        for element in value {
            let val = match_param(key.as_str(), element.as_str());
            debug!("Query parsing | \n\tKey: {key}\n\tValue:{element}");
            res.insert(key, val);
        }
    }
    serde_json::to_value(res).unwrap()
}

/* статистика обхвата члена */
fn match_param(key: &str, value: &str) -> serde_json::Value {
    if value != "1" && key != "diname" {
        return Value::Null;
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
    let system = System::new_all();
    let disks = Disks::new_with_refreshed_list();

    match key {
        "siname" => Value::String(System::name().unwrap()),
        "sikernel_version" => Value::String(System::kernel_version().unwrap()),
        "sios_version" => Value::String(System::os_version().unwrap()),
        "sihostname" => Value::String(System::host_name().unwrap()),
        "mitotal" => Value::Number(Number::from(system.total_memory())),
        "miused" => Value::Number(Number::from(system.used_memory())),
        "miswap_total" => Value::Number(Number::from(system.total_swap())),
        "miswap_used" => Value::Number(Number::from(system.used_swap())),
        "didisks" => {
            let mut disk_map: Map<String, Value> = Map::new();
            for disk in &disks {
                // FIXME: как будто говнокод. мне кажется,можно сделать лучше
                let mut disk_info: Map<String, Value> = Map::new();
                disk_info.insert("name".to_string(), json!(disk.name().to_str()));
                disk_info.insert("total_space".to_string(), json!(disk.total_space()));
                disk_info.insert("available_space".to_string(), json!(disk.available_space()));
                disk_info.insert("kind".to_string(), json!(disk.kind().to_string()));
                disk_info.insert(
                    "file_system".to_string(),
                    json!(disk.file_system().to_str().unwrap().to_string()),
                );
                disk_info.insert("is_removable".to_string(), json!(disk.is_removable()));
                disk_info.insert("used_space".to_string(), json!(disk.total_space() - disk.available_space()));
                disk_map.insert(
                    disk.mount_point().to_str().unwrap().to_string(),
                    Value::Object(disk_info),
                );
            }
            Value::Object(disk_map)
        },
        // TODO: инфу о процессорах, интернет подключениях, процессах?? я хз, почему бы и нет
        // еще вот такой прикол, типа /fetch/?diname="/dev/sda1"&ditotal_space=1
        // response:
        // {
        //  ditotal_space: 23592365
        // }
        // чтобы получать инфу конкретно об одном разделе
        _ => Value::Null,
    }
}

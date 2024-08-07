use multimap::MultiMap;
use serde_json::{json, Map, Number, Value};
use std::collections::{HashMap, HashSet};
use sysinfo::{Disks, System};



pub fn new(params: MultiMap<String, String>) -> serde_json::Value {
    let mut res: HashMap<String, Value> = HashMap::new();
    let mut mnt_map: HashMap<String, Value> = HashMap::new();
    let mut part_map: HashMap<String, Value> = HashMap::new();
    let mut mnts = HashSet::new();
    let mut partitions = HashSet::new();

    for (key, values) in params {
        for value in values {
            if key.starts_with("mnti") && key != "mntiall" {
                mnts.insert(value.clone());
            } else if key.starts_with("mnti") && mnts.contains(&value) {
                let mut entry: HashMap<String, Value> = HashMap::new();
                entry.insert(key.clone(), match_param_mount(&value, &key));
                mnt_map.insert(value.clone(), json!(entry));
            } else if key.starts_with("di") && key != "diall" {
                partitions.insert(value.clone());
            } else if key.starts_with("di") && partitions.contains(&value) {
                let mut entry: HashMap<String, Value> = HashMap::new();
                entry.insert(key.clone(), match_param_part(&value, &key));
                part_map.insert(value.clone(), json!(entry));
            } else {
                debug!("Query parsing | \n\tKey: {key}\n\tValue:{value}");
                res.insert(key.clone(), match_param(key.as_str(), value.as_str()));
            }
        }
    }

    (!mnt_map.is_empty()).then(|| res.insert("mntimounts".to_string(), json!(mnt_map)));

    (!part_map.is_empty()).then(|| res.insert("dipartitions".to_string(), json!(part_map)));

    serde_json::to_value(res).unwrap()
}


fn match_param_mount(mount_point: &str, key: &str) -> serde_json::Value {
    let binding = Disks::new_with_refreshed_list();
    let disk = binding
        .into_iter()
        .find(|&disk| disk.mount_point().to_str().unwrap() == mount_point);

    if disk.is_none() { return json!(null); };
    let disk = disk.unwrap();

    match key {
        "mntitotal_space" => json!(disk.total_space()),
        "mntiavailable_space" => json!(disk.available_space()),
        "mntiused_space" => json!(disk.total_space() - disk.available_space()),
        "mntiname" => json!(disk.name().to_str()),
        "mntiis_removable" => json!(disk.is_removable()),
        "mntifile_system" => json!(disk.file_system().to_str().unwrap().to_string()),
        "mntikind" => json!(disk.kind().to_string()),
        _ => json!(null),
    }
}

fn match_param_part(part_name: &str, key: &str) -> serde_json::Value{
    let binding = Disks::new_with_refreshed_list();
    let disk = binding
        .into_iter()
        .find(|&disk| disk.name().to_str().unwrap() == part_name);

    if disk.is_none() { return json!(null); };
    let disk = disk.unwrap();

    match key {
        "ditotal_space" => json!(disk.total_space()),
        "diavailable_space" => json!(disk.available_space()),
        "diused_space" => json!(disk.total_space() - disk.available_space()),
        "diname" => json!(disk.name().to_str()),
        "diis_removable" => json!(disk.is_removable()),
        "difile_system" => json!(disk.file_system().to_str().unwrap().to_string()),
        "dikind" => json!(disk.kind().to_string()),
        _ => json!(null),
    }
}



/* статистика обхвата члена */
fn match_param(key: &str, value: &str) -> serde_json::Value {
    if value != "1" {
        return Value::Null;
    }
    let mut system = System::new_all();
    system.refresh_all();

    {
        // Prefixes list for field names:
        // s - system (basic system info e.g. name or version)
        // m - ram
        // d - drive
        // mnt - mount
        // i - info
        // c - cpu
        //
        // for example:
        //  "siname" means system_info_name
        //  "sikernel_version" means system_info_kernel_version
        match key {
            "siname" => Value::String(System::name().unwrap()),
            "sikernel_version" => Value::String(System::kernel_version().unwrap()),
            "sios_version" => Value::String(System::os_version().unwrap()),
            "sihostname" => Value::String(System::host_name().unwrap()),
            "mitotal" => Value::Number(Number::from(system.total_memory())),
            "miused" => Value::Number(Number::from(system.used_memory())),
            "miswap_total" => Value::Number(Number::from(system.total_swap())),
            "miswap_used" => Value::Number(Number::from(system.used_swap())),
            "mntiall" => {
                let disk_map: Map<String, Value> = Disks::new_with_refreshed_list()
                    .into_iter()
                    .map(|di| {
                        let disk_info = json!({
                            "name": di.name().to_str(),
                            "total_space": di.total_space(),
                            "available_space": di.available_space(),
                            "kind": di.kind().to_string(),
                            "file_system": di.file_system().to_str().unwrap().to_string(),
                            "is_removable": di.is_removable(),
                            "used_space": di.total_space() - di.available_space()
                        });

                        let mount_point = di.mount_point().to_str().unwrap_or("None");
                        (mount_point.to_string(), disk_info)
                    })
                    .collect();
                Value::Object(disk_map)
            }
            "diall" => {
                let disk_map: Map<String, Value> = Disks::new_with_refreshed_list()
                    .into_iter()
                    .map(|di| {
                        let disk_info = json!({
                            "name": di.name().to_str(),
                            "total_space": di.total_space(),
                            "available_space": di.available_space(),
                            "kind": di.kind().to_string(),
                            "file_system": di.file_system().to_str().unwrap().to_string(),
                            "is_removable": di.is_removable(),
                            "used_space": di.total_space() - di.available_space()
                        });

                        let name = di.name().to_str().unwrap_or("None");
                        (name.to_string(), disk_info)
                    })
                    .collect();
                Value::Object(disk_map)
            }
            "cinfo" => {
                let formatted = format!(
                    "{} - ({})",
                    system.cpus()[0].brand(),
                    system.cpus().len()
                );
                Value::String(formatted)
            }

            // TODO: инфу о процессорах, интернет подключениях, процессах?? я хз, почему бы и нет
            _ => Value::Null,
        }
    }
}


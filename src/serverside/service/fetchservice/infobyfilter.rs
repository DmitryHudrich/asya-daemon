use multimap::MultiMap;
use serde_json::{json, Map, Number, Value};
use std::collections::HashMap;
use sysinfo::{Disk, Disks, System};

pub fn new(params: MultiMap<String, String>) -> serde_json::Value {
    let mut res: HashMap<&String, Value> = HashMap::new();
    let mut needed_mnts: Vec<&String> = vec![];
    let mut mnt_map: HashMap<&String, Value> = HashMap::new();

    let mnts: Vec<String> = Disks::new_with_refreshed_list()
                                    .iter()
                                    .filter_map(|disk| disk.mount_point().to_str().map(|x| x.to_string()))
                                    .collect();


    // dbg!{&params};
    for (key, value) in &params {
        for element in value {
            if &key[0..4] == "mnti" && key != &String::from("mntiall") {
                if !needed_mnts.contains(&element) && mnts.contains(element) {
                    needed_mnts.push(element);
                }
                continue;
            }

            // Processing everything else
            let val = match_param(key.as_str(), element.as_str());
            debug!("Query parsing | \n\tKey: {key}\n\tValue:{element}");
            res.insert(key, val);
        }

        // Processing mountpoints
        if !needed_mnts.is_empty() {
            for mnt in &needed_mnts {
                debug!("Processing mountpoint {}", mnt);
                let mut mnt_submap: HashMap<&String, Value> = HashMap::new();
                for (key, value) in &params {
                    for element in value {
                        if &key[0..4] != "mnti" || &element != mnt {
                            continue;
                        };
                        let val = match_param_mount(mnt, key);
                        mnt_submap.insert(key, val);
                    }
                }
                mnt_map.insert(mnt, json!(mnt_submap));
            }
        }
    }


    let binding: &String = &"mntimounts".to_string();
    (!mnt_map.is_empty()).then(|| res.insert(binding, json!(mnt_map)));
    serde_json::to_value(res).unwrap()
}

fn match_param_mount(mount_point: &str, key: &str) -> serde_json::Value {
    let binding = Disks::new_with_refreshed_list();
    let disk = binding
        .into_iter()
        .find(|&disk| disk.mount_point().to_str().unwrap() == mount_point)
        .unwrap();

    // если в квери вводится маунтпоинт, которого не существует, то анврап будет паниковать
    // надо сделать так, чтобы если вводился несуществующий анврап, то можно было вернуть запрос
    // типа так
    //
    // req: /fetch?mntitotal_space=/nonexistant
    //
    // res:
    // {
    //   mntimounts: {
    //     "/nonexistant": {
    //      mntitotal_space: null       в serde нет андефайнеда((()
    //     }
    //   }
    // }

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


/* статистика обхвата члена */
fn match_param(key: &str, value: &str) -> serde_json::Value {
    if value != "1" {
        return Value::Null;
    }
    let system = System::new_all();

    {
        // Prefixes list for field names:
        // s - system (basic system info e.g. name or version)
        // m - ram
        // d - drive
        // mnt - mount
        // i - info
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

            // "mntiall" => {
            //     let mut disk_map: Map<String, Value> = Map::new();
            //     for disk in &Disks::new_with_refreshed_list() {
            //         // FIXME: как будто говнокод. мне кажется,можно сделать лучше
            //         let mut disk_info: Map<String, Value> = Map::new();
            //         disk_info.insert("name".to_string(), json!(disk.name().to_str()));
            //         disk_info.insert("total_space".to_string(), json!(disk.total_space()));
            //         disk_info.insert("available_space".to_string(), json!(disk.available_space()));
            //         disk_info.insert("kind".to_string(), json!(disk.kind().to_string()));
            //         disk_info.insert(
            //             "file_system".to_string(),
            //             json!(disk.file_system().to_str().unwrap().to_string()),
            //         );
            //         disk_info.insert("is_removable".to_string(), json!(disk.is_removable()));
            //         disk_info.insert(
            //             "used_space".to_string(),
            //             json!(disk.total_space() - disk.available_space()),
            //         );
            //         disk_map.insert(
            //             disk.mount_point().to_str().unwrap().to_string(),
            //             Value::Object(disk_info),
            //         );
            //     }
            //     Value::Object(disk_map)
            // }
            // TODO: инфу о процессорах, интернет подключениях, процессах?? я хз, почему бы и нет
            _ => Value::Null,
        }
    }
}

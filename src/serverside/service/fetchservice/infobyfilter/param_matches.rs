use serde_json::{json, Map, Number, Value};
use sysinfo::{Disks, System};



pub fn match_mount(mount_point: &str, key: &str) -> serde_json::Value {
    let binding = Disks::new_with_refreshed_list();
    let disk = binding
        .into_iter()
        .find(|&disk| disk.mount_point().to_str().unwrap() == mount_point);

    if disk.is_none() {
        return json!(null);
    };
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

pub fn match_param_part(part_name: &str, key: &str) -> serde_json::Value {
    let binding = Disks::new_with_refreshed_list();
    let disk = binding
        .into_iter()
        .find(|&disk| disk.name().to_str().unwrap() == part_name);

    if disk.is_none() {
        return json!(null);
    };
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
pub fn match_param(key: &str, value: &str) -> serde_json::Value {
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
                Value::Object(cdiinfo('m'))
            }
            "diall" => {
                Value::Object(cdiinfo('d'))
            }
            "cinfo" => {
                let formatted =
                    format!("{} - ({})", system.cpus()[0].brand(), system.cpus().len());
                Value::String(formatted)
            }

            // TODO: инфу о интернет подключениях, процессах?? я хз, почему бы и нет
            _ => Value::Null,
        }
    }
}

pub fn cdiinfo(x: char) -> Map<String, Value> {
    Disks::new_with_refreshed_list().into_iter()
        .filter_map(|di| {
            let name = di.name().to_str().unwrap_or_default().to_string();

            let mount_point = match x {
                'm' => di.mount_point().to_str().unwrap_or("None").to_string(),
                'd' => name.clone(),
                _ => return None,
            };

            let disk_info = json!({
                "name": name,
                "total_space": di.total_space(),
                "available_space": di.available_space(),
                "kind": di.kind().to_string(),
                "file_system": di.file_system().to_str().unwrap_or_default().to_string(),
                "is_removable": di.is_removable(),
                "used_space": di.total_space() - di.available_space()
            });

            Some((mount_point, disk_info))
        })
        .collect()
}
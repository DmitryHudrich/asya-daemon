use multimap::MultiMap;
use serde_json::{json, Number, Value};
use sysinfo::System;

pub fn new(params: MultiMap<String, String>) -> serde_json::Value {
    let mut res = MultiMap::new();
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
    if value != "1" {
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

    match key {
        "siname" => Value::String(System::name().unwrap()),
        "sikernel_version" => Value::String(System::kernel_version().unwrap()),
        "sios_version" => Value::String(System::os_version().unwrap()),
        "sihostname" => Value::String(System::host_name().unwrap()),
        "mitotal" => Value::Number(Number::from(system.total_memory())),
        "miused" => Value::Number(Number::from(system.used_memory())),
        "miswap_total" => Value::Number(Number::from(system.total_swap())),
        "miswap_used" => Value::Number(Number::from(system.used_swap())),
        "di_test" => json!({
            "/": {
                "name": "bebra",
                "desc": "sex",
            }
        }),

        _ => Value::Null,
    }
}

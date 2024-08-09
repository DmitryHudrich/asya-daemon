use multimap::MultiMap;
use param_matches::{match_mount, match_param, match_param_part};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

mod param_matches;

pub fn new(params: MultiMap<String, String>) -> serde_json::Value {
    let mut res: HashMap<String, Value> = HashMap::new();
    let mut mnt_map: HashMap<String, Value> = HashMap::new();
    let mut part_map: HashMap<String, Value> = HashMap::new();
    let mut mnts = HashSet::new();
    let mut partitions = HashSet::new();

    // Values vec there for same parametre names with different values.
    for (key, values) in params {
        for value in values {
            if key.starts_with("mnti") && key != "mntiall" {
                mnts.insert(value.clone());
            } else if key.starts_with("mnti") && mnts.contains(&value) {
                let mut entry: HashMap<String, Value> = HashMap::new();
                entry.insert(key.clone(), match_mount(&value, &key));
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


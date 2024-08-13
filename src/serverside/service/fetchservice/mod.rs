use std::collections::HashMap;

use multimap::MultiMap;
use serde::Serialize;
use serde_json::json;

#[macro_use]
mod macro_util;
mod info;
mod param_config;

#[derive(Serialize)]
pub struct ParamInfo {
    primary_type: String,
    secondary_type: String,
    tertiary_type: String,
    property: String,
    #[serde(skip)]
    handler: fn(key: String) -> serde_json::Value,
}

impl ParamInfo {
    fn full_name(&self) -> String {
        let mut res = String::new();
        res.push_str(self.primary_type.as_str());
        res.push_str(self.secondary_type.as_str());
        res.push_str(self.tertiary_type.as_str());
        res.push_str(self.property.as_str());
        res
    }
}

pub fn parse(params: MultiMap<String, String>) -> Vec<serde_json::Value> {
    let mut res = vec![];

    for available_param in param_config::get_available_params() {
        let full_name = available_param.full_name();

        for (param_key, param_values) in &params {
            let mut values_to_res = HashMap::new();
            for param_value in param_values {
                if param_key == &full_name {
                    values_to_res.insert(
                        param_value,
                        (available_param.handler)(param_value.to_owned()),
                    );
                }
            }
            match_param_valuepair(values_to_res, param_key, &mut res);
        }
    }

    res
}

fn match_param_valuepair(
    values_to_res: HashMap<&String, serde_json::Value>,
    param_key: &String,
    res: &mut Vec<serde_json::Value>,
) {
    match values_to_res.len() {
        0 => {
            // Just ignore the param.
        }
        1 => {
            let json_value = json!(
            {
                param_key: {
                    values_to_res.keys().next().unwrap().to_owned():
                        values_to_res.values().next().unwrap().to_owned()
                }
            });
            res.push(json_value);
        }
        2.. => {
            let json_value = json!({param_key: values_to_res});
            res.push(json_value);
        }
    }
}

/*
⣿⣿⣿⣿⣿⡿⠟⠛⠛⢿⠿⠟⠛⠛⠻⢿⣿⣿⡿⠛⠻⢿⣿⣿⣿
⣿⣿⣿⣿⠏⠀⣰⡶⠀⣸⡦⠀⢰⣶⠄⢸⣿⠏⠀⣠⡆⠀⢻⣿⣿
⣿⣿⣿⠏⠀⣼⣿⣿⣿⣿⠃⠀⠛⠁⣀⣾⠏⠀⣼⣿⡇⠀⣼⣿⣿
⣿⣿⡟⠀⢸⣿⣿⣿⣿⡟⠀⣰⣶⠀⢸⡏⠀⣼⣿⡟⠀⢠⣿⣿⣿
⣿⣿⣇⠀⠸⡿⠟⢻⠏⠀⠘⠛⠁⣠⣾⣇⠀⠿⠏⠀⣠⣿⣿⣿⣿
⣿⣿⣿⣷⣤⣤⣶⣾⣷⣴⣦⣤⣶⣿⣿⣿⣦⣤⣴⣾⣿⣿⣿⣿⣿
*/

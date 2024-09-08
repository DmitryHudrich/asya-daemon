use std::{collections::HashMap, ops::Deref};

use multimap::MultiMap;
use serde::Serialize;
use serde_json::{json, Value};

#[macro_use]
mod macro_util;
mod param_config;

mod info;

#[derive(Serialize, Debug, Clone)]
pub struct ParamInfo {
    primary_type: String,
    secondary_type: String,
    tertiary_type: String,
    property: String,
    #[serde(skip)]
    handler: fn(key: String) -> Value,
}

impl Default for ParamInfo {
    fn default() -> Self {
        ParamInfo {
            primary_type: String::new(),
            secondary_type: String::new(),
            tertiary_type: String::new(),
            property: String::new(),
            handler: |_| Value::Null,
        }
    }
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

pub fn parse(params: Vec<(String, String)>) -> HashMap<String, Value> {
    let mut res: HashMap<String, Value> = HashMap::new();
    for (key, value) in &params {
        // 0) If value is "0", skip
        if value == "0" {
            continue;
        }

        // 1) Check if key is valid and get param handler
        let param_handler = param_config::get_available_params()
            .into_iter()
            .find_map(|param| if param.full_name() == *key { Some(param.handler) } else { None });

        // If param handler is not found, insert null and continue
        if let Some(handler) = param_handler {
            // 2) Check if result should be single value or object
            let data = if value.is_empty() || value == "1" {
                // Single Value
                handler(key.clone())
            } else {
                // Object: Gather all pairs that have the same key
                let pairs = params.iter()
                    .filter(|(k, _)| k == key)
                    .collect::<Vec<_>>();

                let tmp: HashMap<String, Value> = pairs
                    .into_iter()
                    .map(|(_, v)| (v.to_string(), handler(v.to_string())))
                    .collect();

                json!(tmp)
            };

            // 3) Insert the processed data
            res.insert(key.clone(), data);
        } else {
            res.insert(key.clone(), Value::Null);
        }
    }

    res
}

/*
⣿⣿⣿⣿⣿⡿⠟⠛⠛⢿⠿⠟⠛⠛⠻⢿⣿⣿⡿⠛⠻⢿⣿⣿⣿
⣿⣿⣿⣿⠏⠀⣰⡶⠀⣸⡦⠀⢰⣶⠄⢸⣿⠏⠀⣠⡆⠀⢻⣿⣿
⣿⣿⣿⠏⠀⣼⣿⣿⣿⣿⠃⠀⠛⠁⣀⣾⠏⠀⣼⣿⡇⠀⣼⣿⣿
⣿⣿⡟⠀⢸⣿⣿⣿⣿⡟⠀⣰⣶⠀⢸⡏⠀⣼⣿⡟⠀⢠⣿⣿⣿
⣿⣿⣇⠀⠸⡿⠟⢻⠏⠀⠘⠛⠁⣠⣾⣇⠀⠿⠏⠀⣠⣿⣿⣿⣿
⣿⣿⣿⣷⣤⣤⣶⣾⣷⣴⣦⣤⣶⣿⣿⣿⣦⣤⣴⣾⣿⣿⣿⣿⣿
*/

/*
               +
               #
              ###
             #####
             ######
            ; #####;
           +##.#####
          +##########
         #############;
        ###############+
       #######   #######
     .######;     ;###;`".
    .#######;     ;#####.
    #########.   .########`
   ######'           '######
  ;####                 ####;
  ##'                     '##
 #'                         `#

          =АР ЧЛИНУКС=
*/

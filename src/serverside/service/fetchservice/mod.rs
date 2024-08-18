use std::collections::HashMap;

use multimap::MultiMap;
use serde::Serialize;
use serde_json::json;

#[macro_use]
mod macro_util;
mod info;
mod param_config;

type Json = serde_json::Value;

#[derive(Serialize, Debug, Clone)]
pub struct ParamInfo {
    primary_type: String,
    secondary_type: String,
    tertiary_type: String,
    property: String,
    #[serde(skip)]
    handler: fn(key: String) -> Json,
}

impl Default for ParamInfo {
    fn default() -> Self {
        ParamInfo {
            primary_type: String::new(),
            secondary_type: String::new(),
            tertiary_type: String::new(),
            property: String::new(),
            handler: |_| Json::Null,
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

// I made it so you pass a map of String-Vec<String> and the function
// returns a map of String-Value.
// So you get 1:1 correspondence of keys (except of invalid keys
// which are filtered out)
//
// eg. you pass a map with keys: issys_name, ismnt_kind, bebra
// the functions returns a map with keys: issys_name, ismnt_kind
pub fn parse(params: MultiMap<String, String>) -> HashMap<String, Json> {
    let mut res: HashMap<String, Json> = HashMap::new();

    'outer: for (param_key, param_values) in params {
        let mut matching_param = None;
        for param in param_config::get_available_params() {
            if param.full_name() == param_key {
                matching_param = Some(param)
            }
        }

        // If a param given in query is invalid, then skip
        if matching_param.is_none() {
            continue;
        }

        let handler = matching_param.unwrap_or_default().handler;

        let mut sub_res = None;
        let mut tmp = HashMap::new();

        for value in param_values {
            if let Ok(val) = value.parse::<u32>() {
                if val == 1 {
                    sub_res = Some(handler(param_key.to_string()));
                    continue;
                }
                // if value is 0 or any other number, then skip
                continue 'outer;
            }

            tmp.insert(value.to_string(), handler(value.to_string()));
            sub_res = Some(json!(tmp));
        }
        res.insert(param_key, json!(sub_res.unwrap_or_default()));
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

use serde_json::json;

use crate::service::fetchservice::info;

use super::ParamInfo;

const T_INFO: &str = "i";
// const T_COMMAND: &str = "c";

const ST_SOFTWARE: &str = "s";
const ST_HARDWARE: &str = "h";

const DEST_MNT: &str = "mnt";
const DEST_DRIVE: &str = "drive";

pub fn get_available_params() -> Vec<ParamInfo> {
    paraminfo!(
        T_INFO: [
            ST_SOFTWARE: [
                DEST_MNT: [
                    "diskname", info::software::mnt::get_diskname_by_mountpoint;
                    "size", |_| json!("");
                ];
            ];
            ST_HARDWARE: [
                DEST_DRIVE: [
                     "name", |_| json!("");
                ]
            ];
        ];
    )
}

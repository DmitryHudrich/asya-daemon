use crate::service::fetchservice::info;

use super::ParamInfo;

const T_INFO: &str = "i";
const T_COMMAND: &str = "c";

const ST_SOFTWARE: &str = "s";
const ST_HARDWARE: &str = "h";

const DEST_MNT: &str = "mnt";
const DEST_DRIVE: &str = "drv";
const DEST_RAM: &str = "ram";
const DEST_CPU: &str = "cpu";
const DEST_SYSTEM: &str = "sys";

// TODO:
// fix this
//  [
//   {
//     "issys_name": {
//       "1": "EndeavourOS"
//     }
//   },
//   {
//     "issys_kernelv": {
//       "1": "6.10.4-arch2-1"
//     }
//   },
//   {
//     "issys_hostname": {
//       "1": "endvr"
//     }
//   },
//   {
//     "issys_osv": {
//       "1": "rolling"
//     }
//   },
//   {
//     "issys_osvlong": {
//       "1": "Linux rolling EndeavourOS"
//     }
//   },
//   {
//     "issys_uptime": {
//       "1": 6633
//     }
//   },
//   {
//     "issys_distroid": {
//       "1": "endeavouros"
//     }
//   }
//  ]
pub fn get_available_params() -> Vec<ParamInfo> {
    paraminfo!(
        T_INFO: [
            ST_SOFTWARE: [
                DEST_MNT: [
                    "_drive", info::software::mnt::get_drive;
                    "_totalspc", info::software::mnt::get_total_space;
                    "_freespc", info::software::mnt::get_available_space;
                    "_usedspc", info::software::mnt::get_used_space;
                    "_kind", info::software::mnt::get_kind;
                    "_fs", info::software::mnt::get_file_system;
                    "_removable", info::software::mnt::get_is_removable;
                ];
                DEST_SYSTEM: [
                    "_name", info::software::sys::get_name;
                    "_kernelv", info::software::sys::get_kernel_version;
                    "_hostname", info::software::sys::get_host;
                    "_osv", info::software::sys::get_os_version;
                    "_osvlong", info::software::sys::get_long_os_version;
                    "_uptime", info::software::sys::get_uptime_seconds;
                    "_distroid", info::software::sys::get_distro_id;
                ];
                DEST_WTR: [
                    "_weather", info::software::wtr::get_wthr
                ]
            ];
            ST_HARDWARE: [
                DEST_DRIVE: [
                    "_mount", info::hardware::drv::get_mount;
                    "_totalspc", info::hardware::drv::get_total_space;
                    "_freespc", info::hardware::drv::get_available_space;
                    "_usedspc", info::hardware::drv::get_used_space;
                    "_kind", info::hardware::drv::get_kind;
                    "_fs", info::hardware::drv::get_file_system;
                    "_removable", info::hardware::drv::get_is_removable;
                ];
                DEST_RAM: [
                    "_totalmem", info::hardware::ram::get_total_memory;
                    "_usedmem", info::hardware::ram::get_used_memory;
                    "_freemem", info::hardware::ram::get_free_memory;
                    "_availmem", info::hardware::ram::get_available_memory;
                    "_totalswp", info::hardware::ram::get_total_swap;
                    "_freeswp", info::hardware::ram::get_free_swap;
                    "_usedswp", info::hardware::ram::get_used_swap;
                ];
                DEST_CPU: [
                     "_name", info::hardware::cpu::get_name;
                     "_cores", info::hardware::cpu::get_core_count;
                     "_brand", info::hardware::cpu::get_brand;
                     "_vendor", info::hardware::cpu::get_vendor;
                     "_usage", info::hardware::cpu::get_global_usage;
                     "_freq", info::hardware::cpu::get_frequency;
                ]
            ];
        ];
    )
}

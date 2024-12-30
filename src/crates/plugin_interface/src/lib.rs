use std::ffi::{c_char, c_int};

pub type PluginInfo = unsafe extern fn() -> *const PluginInformation;

pub type Callback = *const unsafe extern fn(*const c_char);

#[repr(C)]
pub struct PluginInformation {
    pub name: *const c_char,
    pub callback: Callback,
    pub total_callbacks: c_int,
}

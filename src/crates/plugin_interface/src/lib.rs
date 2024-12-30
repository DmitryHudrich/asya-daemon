use std::ffi::{c_char, c_void};

pub type EventCallbalck = unsafe extern "C" fn(EventState) -> OutputState;
pub type InitCallback = unsafe extern "C" fn() -> OutputState;

pub type PluginInfo = unsafe extern fn() -> *const PluginInformation;
pub type ListenCallback = *const unsafe extern fn(EventState) -> OutputState;

#[repr(C)]
pub struct EventState {
    pub state: *const c_void,
    pub event: *const c_char,
}

#[repr(C)]
pub struct InputState {
    pub state: *const c_void
}

#[repr(C)]
pub struct OutputState {
    pub state: *const c_void
}

#[repr(C)]
#[derive(Debug)]
pub struct PluginInformation {
    pub name: *const c_char,
    pub event_callback: EventCallbalck,
    pub init_callback: InitCallback,
}

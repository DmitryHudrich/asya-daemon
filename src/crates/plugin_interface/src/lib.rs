use std::ffi::{c_char, c_void};

pub type EventCallbalck = unsafe extern "C" fn(*const EventState);
pub type ExecuteCallback = unsafe extern "C" fn(*mut State);
pub type InitCallback = unsafe extern "C" fn() -> *mut State;

pub type PluginInfoCallback = unsafe extern "C" fn() -> *const PluginInformation;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EventState {
    pub state: *const State,
    pub event: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct State {
    pub published_event: *mut c_char,
    pub readable_message: *mut c_char,
    pub human_request: *mut c_char,
    pub data: *const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct PluginInformation {
    pub name: *const c_char,
    pub event_callback: EventCallbalck,
    pub init_callback: InitCallback,
    pub execute_callback: ExecuteCallback,
}

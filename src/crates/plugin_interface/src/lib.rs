use std::ffi::{c_char, c_int, CString};

#[repr(C)]
pub struct PluginInformation {
    pub name: *const c_char,
    pub callback: *const unsafe extern fn(*const Event),
    pub total_callbacks: c_int,
}

pub type PluginInfo = unsafe extern "C" fn() -> *const PluginInformation;

#[repr(C)]
pub struct Event {
    pub name: CString,
    pub invariant: Invariant,
    pub available_invariants: *const InvariantDeclaration,
    pub available_invariants_len: usize,
}

#[repr(C)]
pub struct InvariantDeclaration {
    pub name: CString,
    pub fields: *const CString,
    pub fields_len: usize,
}

#[repr(C)]
pub struct Invariant {
    pub name: CString,
    pub fields: *const KeyValue,
    pub fields_len: usize,
}

#[repr(C)]
pub struct KeyValue {
    pub key: CString,
    pub value: CString,
}

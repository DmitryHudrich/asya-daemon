use serde_json::{json, Value};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};


pub fn get_total_memory(_: String) -> Value {
    json!(system(|sys| sys.total_memory()))
}

pub fn get_used_memory(_: String) -> Value {
    json!(system(|sys| sys.used_memory()))
}

pub fn get_free_memory(_: String) -> Value {
    json!(system(|sys| sys.free_memory()))
}

pub fn get_available_memory(_: String) -> Value {
    json!(system(|sys| sys.available_memory()))
}

pub fn get_total_swap(_: String) -> Value {
    json!(system(|sys| sys.total_swap()))
}

pub fn get_used_swap(_: String) -> Value {
    json!(system(|sys| sys.used_swap()))
}

pub fn get_free_swap(_: String) -> Value {
    json!(system(|sys| sys.free_swap()))
}

fn system<T, F>(f: T) -> F
where T: FnOnce(&sysinfo::System) -> F
{
    f(&System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram().with_swap()),
    ))
}

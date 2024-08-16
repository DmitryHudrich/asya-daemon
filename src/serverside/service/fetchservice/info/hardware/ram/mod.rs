use serde_json::json;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

type Json = serde_json::Value;

pub fn get_total_memory(_: String) -> Json {
    json!(system().total_memory())
}

pub fn get_used_memory(_: String) -> Json {
   json!(system().used_memory())
}

pub fn get_free_memory(_: String) -> Json {
   json!(system().free_memory())
}

pub fn get_available_memory(_: String) -> Json {
   json!(system().available_memory())
}

pub fn get_total_swap(_: String) -> Json {
   json!(system().total_swap())
}

pub fn get_used_swap(_: String) -> Json {
   json!(system().used_swap())
}

pub fn get_free_swap(_: String) -> Json {
   json!(system().free_swap())
}

fn system() -> sysinfo::System {
    System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram().with_swap()),
    )
}
use serde_json::json;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

type Json = serde_json::Value;

pub fn get_total_memory(_: String) -> Json {
    let sys = system();
    let res = sys.total_memory();
    json!(res)
}

pub fn get_used_memory(_: String) -> Json {
    let sys = system();
    let res = sys.used_memory();
    json!(res)
}

pub fn get_free_memory(_: String) -> Json {
    let sys = system();
    let res = sys.free_memory();
    json!(res)
}

pub fn get_available_memory(_: String) -> Json {
    let sys = system();
    let res = sys.available_memory();
    json!(res)
}

pub fn get_total_swap(_: String) -> Json {
    let sys = system();
    let res = sys.total_swap();
    json!(res)
}

pub fn get_used_swap(_: String) -> Json {
    let sys = system();
    let res = sys.used_swap();
    json!(res)
}

pub fn get_free_swap(_: String) -> Json {
    let sys = system();
    let res = sys.free_swap();
    json!(res)
}

fn system() -> sysinfo::System {
    System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram().with_swap()),
    )
}

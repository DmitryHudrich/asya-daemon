use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::fetch_dto::RamFetch;

fn get_total_memory() -> Option<u64> {
    system(|sys| sys.total_memory())
}

fn get_used_memory() -> Option<u64> {
    system(|sys| sys.used_memory())
}

fn get_free_memory() -> Option<u64> {
    system(|sys| sys.free_memory())
}

fn get_available_memory() -> Option<u64> {
    system(|sys| sys.available_memory())
}

fn get_total_swap() -> Option<u64> {
    system(|sys| sys.total_swap())
}

fn get_used_swap() -> Option<u64> {
    system(|sys| sys.used_swap())
}

fn get_free_swap() -> Option<u64> {
    system(|sys| sys.free_swap())
}

fn system<T, F>(f: T) -> Option<F>
where
    T: FnOnce(&System) -> F,
{
    Some(f(&System::new_with_specifics(RefreshKind::new().with_memory(
        MemoryRefreshKind::new().with_ram().with_swap(),
    ))))
}

pub fn get_ram_fetch() -> Option<RamFetch> {
  Some(RamFetch {
    total_memory: get_total_memory(),
    used_memory: get_used_memory(),
    free_memory: get_free_memory(),
    available_memory: get_available_memory(),
    total_swap: get_total_swap(),
    used_swap: get_used_swap(),
    free_swap: get_free_swap()
  })
}

use serde_json::{json, Value};
use sysinfo::{CpuRefreshKind, RefreshKind, System};


pub fn get_global_usage(_: String) -> Value {
    json!(system(|sys| sys.global_cpu_usage()))
}

pub fn get_brand(_: String) -> Value {
    json!(system(|sys| sys.cpus()[0].brand().to_string()))
}

pub fn get_core_count(_: String) -> Value {
    json!(system(|sys| sys.cpus().len()))
}

pub fn get_vendor(_: String) -> Value {
    json!(system(|sys| sys.cpus()[0].vendor_id().to_string()))
}

pub fn get_name(_: String) -> Value {
    json!(system(|sys| sys.cpus()[0].name().to_string()))
}

// TODO:
// я не уверен, что это вообще возвращает. каждый раз, когда я собираю
// данные о частоте процессора, она всегда разная. как это работает?
pub fn get_frequency(_: String) -> Value {
    json!(system(|sys| sys.cpus()[0].frequency()))
}

fn system<T, F>(f: T) -> F
where
    T: FnOnce(&sysinfo::System) -> F
{
    f(&System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything())))
}

// FIXME:
//  функция sys.cpus() возвращает вектор ЯДЕР ПРОЦЕССОРОВ,
// и у каждого ядра есть своя загруженность, частота и прочее.
// А еще может быть такое, что у пользователя 2 процессора, и тогда
// бренд, производитель, имя и т.д. тоже могут отличаться.
//  Можно сделать так, чтобы в запросе можно было указывать ядро
// инфу о котором нам надо получить. Если ядро не указано, то по дефолту
// это нулевое ядро.

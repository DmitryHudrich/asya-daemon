// unfortunately we can't put this file to the 'usecases' because this causing cycle dependencies
#[derive(Clone)]
pub enum WorkerState {
    Empty,
    Counter { count: u64 },
    ProcMonitor { processes: Vec<String> }, // just names for testing. feel free for realize full functionality
    Mouse { x: u64, y: u64 },
    Weather { weather: String, celsius: i64 },
    Timer { current_time: u64 },
}

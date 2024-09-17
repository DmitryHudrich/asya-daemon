// unfortunately we can't put this file to the 'usecases' because this causing cycle dependencies
pub enum WorkerState {
    Empty,
    ProcMonitor { processes: Vec<String> }, // just names for testing. feel free for realize full functionality
    Mouse { x: u64, y: u64 },
    Weather { weather: String, celsius: i64 },
    Timer { current_time: u64 },
}


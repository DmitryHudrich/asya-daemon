use std::sync::atomic::{AtomicBool, Ordering};

use services::services::info::hardware::{cpu, ram};
use shared::event_system;
use tokio::task;

#[derive(Debug, parse_display::Display)]
pub enum HardwareStatus {
    #[display("CPU: {cpu_usage}%, MEM: {mem_usage}%")]
    Ok { cpu_usage: f32, mem_usage: u64 },
}

pub struct SystemMonitor;

impl SystemMonitor {
    pub async fn start_basic_monitoring() {
        static IS_RUNNING: AtomicBool = AtomicBool::new(false);
        if IS_RUNNING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            task::spawn(async {
                loop {
                    let cpu_usage = cpu::get_global_usage("".to_string());
                    let mem_usage = ram::get_used_memory("".to_string());
                    event_system::publish(HardwareStatus::Ok {
                        cpu_usage: cpu_usage.unwrap_or(0.0),
                        mem_usage: mem_usage.unwrap_or(0),
                    })
                    .await;
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            });
        }
    }
}

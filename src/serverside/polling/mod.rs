use std::ops::DerefMut;

use lazy_static::lazy_static;
use queues::{queue, IsQueue, Queue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

lazy_static! {
    static ref COMMAND_QUEUE: Mutex<Queue<Command>> = Mutex::new(queue![]);
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Command {
    status: u8,
    content: Option<CommandContent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandContent {
    command: String,
    additional: Value,
}

pub async fn return_command() -> Command {
    let mut mg = COMMAND_QUEUE.lock().await;
    let command_queue = mg.deref_mut();
    match command_queue.remove() {
        Ok(v) => v,
        Err(_) => Command::default(),
    }
}

pub fn send_command() {}

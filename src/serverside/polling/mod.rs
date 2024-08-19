use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Signal {
    status: u8,
    content: Option<SignalContent>,
}

impl Signal {
    pub async fn info(content: SignalContent) {
        signal(20, content).await
    }

    pub async fn error(content: SignalContent) {
        signal(10, content).await
    }

    pub async fn command(content: SignalContent) {
        signal(0, content).await
    }
}

async fn signal(status: u8, content: SignalContent) {
    let signal = Signal {
        status,
        content: Some(content),
    };

    send_command(signal).await;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignalContent {
    command: String,
    additional: Value,
}

impl SignalContent {
    pub fn new(command: String) -> SignalContent {
        SignalContent {
            command,
            additional: Value::Null,
        }
    }
}

async fn send_command(signal: Signal) {
      
}

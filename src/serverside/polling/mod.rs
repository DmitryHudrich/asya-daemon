use lazy_static::lazy_static;
use multimap::MultiMap;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

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

pub struct SignalContent {
    command: String,
    additional: MultiMap<String, String>,
}

impl SignalContent {
    pub fn new(command: String) -> SignalContent {
        SignalContent {
            command,
            additional: MultiMap::new(),
        }
    }
}

async fn send_command(signal: Signal) {}

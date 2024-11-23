use tokio::sync::RwLock;

pub mod configuration;
pub mod logging_engine;
pub mod utils;
pub mod types;

pub static ASYA_STATUS: RwLock<AsyaStatus> = RwLock::const_new(AsyaStatus {
    tgtoken_obtained: false,
});

pub struct AsyaStatus {
    pub tgtoken_obtained: bool,
}

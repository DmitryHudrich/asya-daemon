use crate::configuration;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

lazy_static! {
    static ref ASYA_STATUS: RwLock<AsyaStatus> = RwLock::const_new(AsyaStatus::default());
}

#[derive(Debug, Default)]
struct AsyaStatus {
    tg_accepted_users: Option<Vec<String>>,
    tgtoken: Option<String>,
    http_port: Option<u16>,
    grpc_port: Option<u16>,
    logging_place: Option<bool>,
    logging_level: Option<log::LevelFilter>,
    logging_folder: Option<String>,
    logging_filescount: Option<usize>,
    logging_stdout: Option<bool>,
    mistral_token: Option<String>,
    proxy_addr: Option<String>,
    is_mistral_token_obtained: bool,
    recognize_method: Option<configuration::AiRecognizeMethod>
}

pub async fn init_state() {
    let mut asya_status = ASYA_STATUS.write().await;
    let status = AsyaStatus {
        tg_accepted_users: configuration::get().telegram().accepted_users(),
        tgtoken: configuration::get().telegram().token(),
        http_port: configuration::get().net().http_port(),
        grpc_port: None,
        logging_place: configuration::get().logging().place(),
        logging_level: configuration::get().logging().level(),
        logging_folder: configuration::get().logging().folder(),
        logging_filescount: None,
        logging_stdout: configuration::get().logging().stdout(),
        mistral_token: configuration::get().ai().groq_token(),
        proxy_addr: configuration::get().net().proxy_addr(),
        is_mistral_token_obtained: configuration::get().ai().groq_token().is_some(),
        recognize_method: configuration::get().ai().recognize_method()
    };
    *asya_status = status;
}

pub async fn get_ai_req_method() -> Option<configuration::AiRecognizeMethod> {
    ASYA_STATUS.read().await.recognize_method.clone()
}

pub async fn is_llm_obtained() -> bool {
    ASYA_STATUS.read().await.is_mistral_token_obtained
}

pub async fn get_proxy_addr() -> Option<String> {
    ASYA_STATUS.read().await.proxy_addr.clone()
}

pub async fn get_mistral_token() -> Option<String> {
    ASYA_STATUS.read().await.mistral_token.clone()
}

pub async fn get_tgtoken() -> Option<String> {
    ASYA_STATUS.read().await.tgtoken.clone()
}

pub async fn get_tg_accepted_users() -> Option<Vec<String>> {
    ASYA_STATUS.read().await.tg_accepted_users.clone()
}

pub async fn get_http_port() -> Option<u16> {
    ASYA_STATUS.read().await.http_port
}

pub async fn get_grpc_port() -> Option<u16> {
    ASYA_STATUS.read().await.grpc_port
}

pub async fn get_logging_place() -> Option<bool> {
    ASYA_STATUS.read().await.logging_place
}

pub async fn get_logging_level() -> Option<log::LevelFilter> {
    ASYA_STATUS.read().await.logging_level
}

pub async fn get_logging_folder() -> Option<String> {
    ASYA_STATUS.read().await.logging_folder.clone()
}

pub async fn get_logging_filescount() -> Option<usize> {
    ASYA_STATUS.read().await.logging_filescount
}

pub async fn get_logging_stdout() -> Option<bool> {
    ASYA_STATUS.read().await.logging_stdout
}

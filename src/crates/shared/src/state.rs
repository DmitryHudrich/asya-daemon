use crate::{configuration::get, types::AiRecognizeMethod};

pub fn get_autolaunch_alta_s() -> Option<bool> {
    get("/telegram/accepted_users")
}

pub fn get_alta_s_path() -> Option<String> {
    get("/ai/alta_s_path")
}

pub fn get_alta_s_addr() -> Option<String> {
    get("/ai/alta_s_addr")
}

pub fn get_ai_req_method() -> Option<AiRecognizeMethod> {
    get("/ai/recognize_method")
}

pub fn is_llm_obtained() -> bool {
    get_mistral_token().is_some()
}

pub fn get_proxy_addr() -> Option<String> {
    get("/net/proxy_addr")
}

pub fn get_mistral_token() -> Option<String> {
    get("/ai/groq_token")
}

pub fn get_tgtoken() -> Option<String> {
    get("/telegram/token")
}

pub fn get_tg_accepted_users() -> Option<Vec<String>> {
    get("/telegram/accepted_users")
}

pub fn get_http_port() -> Option<u16> {
    get("/net/http_port")
}

pub fn get_grpc_port() -> Option<u16> {
    get("/net/grpc_port")
}

pub fn get_logging_place() -> Option<bool> {
    get("/logging/place")
}

pub fn get_logging_level() -> Option<log::LevelFilter> {
    get("/logging/level")
}

pub fn get_logging_folder() -> Option<String> {
    get("/logging/folder")
}

pub fn get_logging_filescount() -> Option<usize> {
    get("/logging/filescount")
}

pub fn get_logging_stdout() -> Option<bool> {
    get("/logging/stdout")
}

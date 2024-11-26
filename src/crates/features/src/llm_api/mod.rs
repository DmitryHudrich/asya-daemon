use groq_api::send_to_groq;
use shared::{state, types::AiRecognizeMethod};

mod groq_api;

// todo: покрыть все ошибки, а не те которые мне по кайфу щас
#[derive(Debug)]
pub enum AiRequestError {
    GroqApiKey,
    GroqRequest,
}

// todo: rewrite to result
/// Returns `None` if token unspecified or ошибка случилась
pub async fn send_request(req: String) -> Result<String, AiRequestError> {
    match state::get_ai_req_method().expect("ai_req_method unspecified.") {
        AiRecognizeMethod::Groq => send_to_groq(req).await,
        AiRecognizeMethod::AltaS => todo!(),
        AiRecognizeMethod::None => Ok(req), // nothing for recognize, so just return command
    }
}

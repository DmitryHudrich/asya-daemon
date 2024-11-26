use alta_s_api::send_to_altas;
use groq_api::send_to_groq;
use reqwest::Client;
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
        AiRecognizeMethod::AltaS => send_to_altas(req).await,
        AiRecognizeMethod::None => Ok(req), // nothing for recognize, so just return command
    }
}

async fn request(
    client: Client,
    url: &str,
    api_key: String,
    body: serde_json::Value,
) -> reqwest::Response {
    client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .unwrap()
}

mod alta_s_api {
    use reqwest::Client;
    use shared::{state, utils::file_utils};

    use super::AiRequestError;

    pub async fn send_to_altas(req: String) -> Result<String, AiRequestError> {
        let client = Client::new();
        let url = state::get_alta_s_addr().expect("AltaS url isn't set");
        let response = construct_and_send_reqwest(req, client, url.as_str())
            .await
            .expect("AltaS response");
        Ok(response)
    }

    async fn construct_and_send_reqwest(req: String, client: Client, url: &str) -> Option<String> {
        let response = client
            .get(url)
            .query(&[("text", req.as_str())])
            .send()
            .await
            .expect("altas response");

        let response_res = response.text().await;
        let temp = &response_res.unwrap();
        let response_text = temp.as_str();
        file_utils::get_json_value(response_text, "/result/answer")
    }
}

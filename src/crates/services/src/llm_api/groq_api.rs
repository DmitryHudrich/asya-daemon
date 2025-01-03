use reqwest::Client;
use serde_json::json;
use shared::{configuration::CONFIG, serde_extensions::get_json_value};

use super::{request, AiRequestError};

pub async fn send_to_groq(req: String) -> Result<String, AiRequestError> {
    let client = Client::new();
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let api_key = CONFIG.ai.groq_token.clone();

    construct_and_send_reqwest(req, client, url, api_key)
        .await
        .ok_or(AiRequestError::GroqRequest)
}

async fn construct_and_send_reqwest(
    req: String,
    client: Client,
    url: &str,
    api_key: String,
) -> Option<String> {
    let body = json!({
        "messages": [
            {
                "role": "user",
                "content": req
            }
        ],
        "model": "llama-3.3-70b-versatile",
        "temperature": 0.7
    });

    let response = request(client, url, api_key, body).await;

    get_json_value(
        &response.text().await.unwrap(),
        "/choices/0/message/content",
    )
    // .map(|s| s.replace("\\", ""))
}

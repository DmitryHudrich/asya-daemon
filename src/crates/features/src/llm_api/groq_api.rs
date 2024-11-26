use reqwest::{Client, Proxy};
use serde_json::json;
use shared::{state, utils::file_utils};

use super::{request, AiRequestError};

pub async fn send_to_groq(req: String) -> Result<String, AiRequestError> {
    let client = Client::new();
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let api_key = state::get_mistral_token();
    if let Some(api_key) = api_key {
        if let Some(response) = construct_and_send_reqwest(req, client, url, api_key).await {
            Ok(response)
        } else {
            Err(AiRequestError::GroqRequest)
        }
    } else {
        Err(AiRequestError::GroqApiKey)
    }
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
        "model": "llama-3.1-70b-versatile",
        "temperature": 0.7
    });

    let response = request(client, url, api_key, body).await;

    let response_res = response.text().await;
    let temp = &response_res.unwrap();
    let response_text = temp.as_str();
    file_utils::get_json_value(response_text, "/choices/0/message/content")
}

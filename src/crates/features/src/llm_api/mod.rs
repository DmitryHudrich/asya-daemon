use reqwest::{Client, Proxy};
use serde_json::json;
use shared::{state, utils::file_utils};

// todo: rewrite to result
/// Returns `None` if token unspecified or ошибка случилась
pub async fn send_request(req: String) -> Option<String> {
    let client = construct_reqwest_client().await;
    let url = "https://api.groq.com/openai/v1/chat/completions";
    let api_key = state::get_mistral_token().await;
    if let Some(api_key) = api_key {
        construct_and_send_reqwest(req, client, url, api_key).await
    } else {
        println!("nenra");
        None
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

async fn construct_reqwest_client() -> Client {
    if let Some(proxy_addr) = state::get_proxy_addr().await {
        let proxy = Proxy::http(proxy_addr).expect("Error while proxy setup.");
        Client::builder()
            .proxy(proxy)
            .build()
            .expect("Error while building client with proxy.")
    } else {
        Client::new()
    }
}

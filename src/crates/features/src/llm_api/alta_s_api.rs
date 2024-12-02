use reqwest::Client;
use shared::{serde_extensions::get_json_value, state};

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

    get_json_value(&response.text().await.unwrap(), "/result/answer")
}

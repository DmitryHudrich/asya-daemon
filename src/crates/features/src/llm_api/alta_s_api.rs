use reqwest::Client;
use shared::{serde_extensions::get_json_value, state};

use super::AiRequestError;

pub async fn send_to_altas(req: String) -> Result<String, AiRequestError> {
    let client = Client::new();
    let url = state::get_alta_s_addr().ok_or(AiRequestError::AltaSUrl)?;

    construct_and_send_reqwest(req, client, url.as_str())
        .await
        .ok_or(AiRequestError::AltaSRequest)
}

async fn construct_and_send_reqwest(req: String, client: Client, url: &str) -> Option<String> {
    let response = client
        .get(url)
        .query(&[("text", req.as_str())])
        .send()
        .await
        .expect("The AltaS response should be received");

    get_json_value(&response.text().await.unwrap(), "/result/answer")
}

mod http;
mod fetch_dto;

pub async fn start() {
    http::start().await;
}

use actix_web::HttpServer;

mod routing;
mod ws_utils;
mod requests;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(routing::route_all)
        .disable_signals()
        .bind(("127.0.0.1", shared::state::get_http_port().await.expect("http port is not defined.")))?
        .run()
        .await
}

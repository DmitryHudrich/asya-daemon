use actix_web::HttpServer;
use shared::configuration::CONFIG;

mod requests;
mod responses;
mod routing;
mod ws_utils;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(routing::route_all)
        .disable_signals()
        .bind(("127.0.0.1", CONFIG.net.http_port))?
        .run()
        .await
}

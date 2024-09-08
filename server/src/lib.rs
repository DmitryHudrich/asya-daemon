use actix_web::HttpServer;
use shared::configuration;

mod routing;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        routing::route_all()
    })
    .disable_signals()
    .bind(("127.0.0.1", configuration::get().net().http_port()))?
    .run()
    .await
}

use actix_web::HttpServer;
mod routing;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        routing::route_all()
    })
    .disable_signals()
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// pub async fn start() {
//     let http = http::start();
//     let ws = ws::start();
//     join!(http, ws);
// }

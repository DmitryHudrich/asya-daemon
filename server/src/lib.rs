use tokio::join;

mod http;
mod ws;

pub async fn start() {
    let http = http::start();
    let ws = ws::start();
    join!(http, ws);
}

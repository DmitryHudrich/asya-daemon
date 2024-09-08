use actix_web::{rt, Error, HttpResponse};
use actix_web::{web, App, HttpRequest, HttpServer};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use prost::Message;
use service::fetches::{self, fetch_dto};

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/echo", web::get().to(echo)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            if let Ok(AggregatedMessage::Text(text)) = msg {
                if text == "ftch" {
                    let response = fetch_dto::SuperFetch {
                        cpu: fetches::hardware::cpu::get_cpu_fetch(),
                        sys: fetches::software::sys::get_sys_fetch(),
                        mnt: fetches::software::mnt::get_mnt_fetch(
                            &req.uri().query().unwrap_or("").to_string(),
                        ),
                        drv: fetches::hardware::drv::get_drv_fetch(
                            &req.uri().query().unwrap_or("").to_string(),
                        ),
                        ram: fetches::hardware::ram::get_ram_fetch(),
                    };
                    // echo binary message
                    session.binary(response.encode_to_vec()).await.unwrap();
                }
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

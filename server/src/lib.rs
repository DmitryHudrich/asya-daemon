use actix_web::{middleware, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use service::services::fetchservice;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
            .route("/hey", web::get().to(|| async { "bebra" }))
            .route("/sex", web::get().to(|| async { "не было" }))
            .route(
                "/fetch",
                web::get().to(|req: HttpRequest| async move {
                    let params =
                        web::Query::<Vec<(String, String)>>::from_query(req.query_string())
                            .unwrap();
                    let res = fetchservice::parse(params.into_inner());
                    HttpResponse::Ok()
                        .body(serde_json::to_string(&res).unwrap())
                }),
            )
            .route("/echo", web::get().to(echo))
    })
    .disable_signals()
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
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

// pub async fn start() {
//     let http = http::start();
//     let ws = ws::start();
//     join!(http, ws);
// }

use std::net::SocketAddr;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Bytes, server::conn::http1, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use middlewares::logging;
use serde::Serialize;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use crate::service::fetchservice::{basicinfo, memoryinfo};

mod middlewares;

pub async fn launch_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening at {}", addr.to_string());

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move {
            let svc = hyper::service::service_fn(router);
            let svc = ServiceBuilder::new()
                .layer_fn(logging::Logger::new)
                .service(svc);
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                warn!("server error: {}", err);
            }
        });
    }
}

async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        /*
            ⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣶⣿⠏⣿⣿⣿⣿⣿⣁⠀⠀⠀⠛⠙⠛⠋      апиха снизу, команда, кайфуйте
            ⡿⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⣰⣿⣿⣿⣿⡄⠘⣿⣿⣿⣿⣷⠄
            ⡇⠀⠀⠀⠀⠀⠀⠀⠸⠇⣼⣿⣿⣿⣿⣿⣷⣄⠘⢿⣿⣿⣿⣅
            ⠁⠀⠀⠀⣴⣿⠀⣐⣣⣸⣿⣿⣿⣿⣿⠟⠛⠛⠀⠌⠻⣿⣿⣿⡄
            ⠀⠀⠀⣶⣮⣽⣰⣿⡿⢿⣿⣿⣿⣿⣿⡀⢿⣤⠄⢠⣄⢹⣿⣿⣿⡆
            ⠀⠀⠀⣿⣿⣿⣿⣿⡘⣿⣿⣿⣿⣿⣿⠿⣶⣶⣾⣿⣿⡆⢻⣿⣿⠃⢠⠖⠛⣛⣷
            ⠀⠀⢸⣿⣿⣿⣿⣿⣿⣾⣿⣿⣿⣿⣿⣿⣮⣝⡻⠿⠿⢃⣄⣭⡟⢀⡎⣰⡶⣪⣿
            ⠀⠀⠘⣿⣿⣿⠟⣛⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⡿⢁⣾⣿⢿⣿⣿⠏
            ⠀⠀⠀⣻⣿⡟⠘⠿⠿⠎⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣵⣿⣿⠧⣷⠟⠁
            ⡇⠀⠀⢹⣿⡧⠀⡀⠀⣀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠋⢰⣿
            ⡇⠀⠀⠀⢻⢰⣿⣶⣿⡿⠿⢂⣿⣿⣿⣿⣿⣿⣿⢿⣻⣿⣿⣿⡏⠀⠀ 
        */

        (&Method::GET, "/fetch") => ok(&basicinfo::BasicInfo::new()),
        (&Method::GET, "/fetch/memory") => ok(&memoryinfo::MemoryInfo::new()),

        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn ok<T>(result: &T) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
where
    T: Serialize,
{
    let response = Response::builder()
        .header("Content-Type", "application/json")
        .body(full(serde_json::to_string(result).unwrap()))
        .unwrap();
    Ok(response)
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

use http_body_util::BodyExt;
use http_body_util::Full;

use http_body_util::Empty;

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper_util::rt::TokioIo;
use prost::Message;
use serde::Serialize;
use std::net::SocketAddr;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tokio::join;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use crate::configuration;
use crate::service::fetchservice;
use middlewares::logging;

mod middlewares;

tonic::include_proto!("fetch_is_cpu");

pub(crate) async fn start() {
    join!(http1_start());
}

async fn http1_start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration::get().net().http_port()));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("HTTP1 Start listening at {}", addr.to_string());
    loop {
        let (stream, _) = listener.accept().await.unwrap();

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

pub(crate) async fn router(
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
        (&Method::GET, "/fetch") => {
            let params = req
                .uri()
                .query()
                .map(|v| form_urlencoded::parse(v.as_bytes()).into_owned().collect())
                .unwrap_or_default();
            ok(&fetchservice::parse(params))
        }
        (&Method::GET, "/prototest") => {
            let a = FrequencyInfo {
                frequency: {
                    let nws = &System::new_with_specifics(
                        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
                    );
                    nws.cpus()[0].frequency()
                },
            };
            let buf = a.encode_to_vec();
            let response = Response::builder()
                .header("Content-Type", "application/x-protobuf")
                .body(full(buf))
                .unwrap();
            Ok(response)
        }
        (&Method::GET, "/ping") => ok(&"pong"),
        // todo (&Method::GET, "/helth") =>
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub(crate) fn ok<T>(result: &T) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
where
    T: Serialize,
{
    let response = Response::builder()
        .header("Content-Type", "application/json")
        .body(full(serde_json::to_string(result).unwrap()))
        .unwrap();
    Ok(response)
}

pub(crate) fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub(crate) fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

use http_body_util::BodyExt;
use http_body_util::Full;

use http_body_util::Empty;

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::body::{Body as HttpBody, Frame};
use hyper::server::conn::http1;
use hyper::server::conn::http2;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper::Version;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use serde::Serialize;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::join;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use crate::configuration;
use crate::service::fetchservice;
use middlewares::logging;

mod middlewares;

pub(crate) async fn start() {
    join!(http1_start(), http2_start());
}

async fn http2_start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1488));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Start listening at {}", addr.to_string());
    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let io = TokioIo::new(stream);
        tokio::spawn(async move {
            let svc = hyper::service::service_fn(router);
            let svc = ServiceBuilder::new()
                .layer_fn(logging::Logger::new)
                .service(svc);
            if let Err(err) = http2::Builder::new(TokioExecutor::new())
                .serve_connection(io, svc)
                .await
            {
                warn!("server error: {}", err);
            }
        });
    }
}

async fn http1_start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration::get().net().http_port()));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Start listening at {}", addr.to_string());
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

pub(crate) struct Body {
    // Our Body type is !Send and !Sync:
    _marker: PhantomData<*const ()>,
    data: Option<Bytes>,
}

impl From<String> for Body {
    fn from(a: String) -> Self {
        Body {
            _marker: PhantomData,
            data: Some(a.into()),
        }
    }
}

impl HttpBody for Body {
    type Data = Bytes;
    type Error = hyper::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        Poll::Ready(self.get_mut().data.take().map(|d| Ok(Frame::data(d))))
    }
}

pub(crate) async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match req.version() {
        Version::HTTP_2 => match (req.method(), req.uri().path()) {
            (&Method::GET, "/ping_http2") => ok(&"ebat v zhopu"),
            (&Method::GET, "/fetch") => {
                let params = req
                    .uri()
                    .query()
                    .map(|v| form_urlencoded::parse(v.as_bytes()).into_owned().collect())
                    .unwrap_or_default();
                ok(&fetchservice::parse(params))
            }
            _ => {
                let mut not_found = Response::new(empty());
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        },
        _ => match (req.method(), req.uri().path()) {
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
            (&Method::GET, "/ping") => ok(&"pong"),
            // todo (&Method::GET, "/helth") =>
            _ => {
                let mut not_found = Response::new(empty());
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        },
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

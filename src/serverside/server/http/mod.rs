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
use hyper_util::rt::TokioIo;
use serde::Serialize;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use crate::configuration;
use crate::service::fetchservice;
use middlewares::logging;

mod middlewares;

#[derive(Clone, Copy, Debug)]
struct LocalExec;

impl<F> hyper::rt::Executor<F> for LocalExec
where
    F: std::future::Future + 'static, // not requiring `Send`
{
    fn execute(&self, fut: F) {
        // This will spawn into the currently running `LocalSet`.
        tokio::task::spawn_local(fut);
    }
}

pub(crate) async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration::get().net().http_port()));
    let listener = TcpListener::bind(addr).await?;

    let http2_addr = SocketAddr::from(([127, 0, 0, 1], 5549));
    let http2_listener = TcpListener::bind(http2_addr).await?;

    info!("Start listening at {}", addr.to_string());
    info!("Start listening HTTP2 at {}", http2_addr.to_string());
    loop {
        let (stream, _) = listener.accept().await?;
        let (http2_stream, _) = http2_listener.accept().await?;

        let io = TokioIo::new(stream);
        let http2_io = TokioIo::new(http2_stream);
        tokio::spawn(async move {
            let svc = hyper::service::service_fn(router);
            let svc = ServiceBuilder::new()
                .layer_fn(logging::Logger::new)
                .service(svc);
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                warn!("server error: {}", err);
            }
        });

        tokio::spawn(async move {
            let svc = hyper::service::service_fn(http2_router);
            let svc = ServiceBuilder::new()
                .layer_fn(logging::Logger::new)
                .service(svc);
            debug!("http2 spawned");
            if let Err(err) = http2::Builder::new(LocalExec)
                .serve_connection(http2_io, svc)
                .await
            {
                warn!("server error: {}", err);
            }
        });
    }
}

struct Body {
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

pub async fn http2_router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/fetch/cpu/frequency") => {
            debug!("http2 cpu freq");
            Ok::<_, hyper::Error>(Response::new(Body::from("BEBRA".to_string())))
        }
        _ => {
            let mut not_found = Response::new(Body::from("".to_string()));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
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
        (&Method::GET, "/ping") => ok(&"pong"),
        // todo (&Method::GET, "/helth") =>
        // (&Method::GET, "/fetch/memory") => ok(&memoryinfo::MemoryInfo::new()),
        // (&Method::GET, "fetch/mounts")
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

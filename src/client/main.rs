use http_body_util::{BodyExt, Empty};
use hyper::{body::Bytes, Request};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    print_fetch().await;
}

async fn print_fetch() {
    let fetch = set_connection("http://localhost:3000/fetch".parse::<hyper::Uri>().unwrap())
        .await
        .unwrap();
    println!("FETCH: {}", fetch);
}

async fn set_connection(url: hyper::Uri) -> Result<String> {
    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<String> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let req = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let res = sender.send_request(req).await?;
    let body_bytes = res.collect().await?.to_bytes();
    Ok(String::from_utf8(body_bytes.to_vec()).unwrap())
}

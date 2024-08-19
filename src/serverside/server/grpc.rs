use main::signal_service_server::{SignalService, SignalServiceServer};
use main::{Signal, SignalContent};

use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod main {
    tonic::include_proto!("signal_service"); // The string specified here must match the proto package name
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let signal = MySignal::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(SignalServiceServer::new(signal))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct MySignal {}

#[tonic::async_trait]
impl SignalService for MySignal {
    async fn send_signal(
        &self,
        request: Request<Signal>, // Accept request of type HelloRequest
    ) -> Result<Response<Signal>, Status> {

        // default yet
        let reply = Signal {
            status: 0,
            content: Some(SignalContent {
                command: String::from("bebra"),
                additional: vec![]
            })
        };

        Ok(Response::new(reply))
    }
}


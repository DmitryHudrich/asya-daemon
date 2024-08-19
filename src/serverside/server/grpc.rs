use main::signal_service_server::{SignalService, SignalServiceServer};
// use main::signal_service_client::SignalServiceClient;
use main::{Signal, SignalContent};

use tokio::join;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod main {
    tonic::include_proto!("signal_service"); // The string specified here must match the proto package name
}

pub async fn start() {
    _ = join!(server())
}

// async fn client() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = SignalServiceClient::connect("http://[::1]:50051").await?;
//
//     let request = tonic::Request::new(Signal {
//             status: 0,
//             content: Some(SignalContent {
//                 command: String::from("bebra"),
//                 additional: vec![],
//             }),
//         });
//
//
//     let response = client.handle_signal(request).await?;
//
//     Ok(())
// }

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let signal = MySignal::default();

    info!("GRPC Server listening on {}", addr);

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
    async fn handle_signal(
        &self,
        _: Request<Signal>, // Accept request of type HelloRequest
    ) -> Result<Response<Signal>, Status> {
        // default yet
        let reply = Signal {
            status: 0,
            endpoint: String::from("bebra"),
            content: Some(SignalContent {
                command: String::from("bebra"),
                additional: vec![],
            }),
        };

        Ok(Response::new(reply))
    }
}

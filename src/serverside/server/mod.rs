use tokio::join;

//use crate::service::fetchservice::{ infobyfilter, memoryinfo};

mod http;
mod middlewares;
mod grpc;

pub async fn launch_server() {
    _ = join!(http::start(), grpc::start());
}


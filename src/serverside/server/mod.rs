use tokio::join;

//use crate::service::fetchservice::{ infobyfilter, memoryinfo};

mod http;

pub async fn launch_server() {
    _ = join!(http::start());
}

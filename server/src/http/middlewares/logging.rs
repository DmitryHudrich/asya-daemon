use hyper::{body::Incoming, service::Service, Request};
use log::*;

#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}
impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}

type Req = Request<Incoming>;

impl<S> Service<Req> for Logger<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, req: Req) -> Self::Future {
        info!(
            "processing request: {method} {uripath}",
            method = req.method(),
            uripath = req.uri().path()
        );
        self.inner.call(req)
    }
}

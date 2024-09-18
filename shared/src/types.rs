use std::{future::Future, pin::Pin};

// this is used for asynchronous initialization and worker operation
pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;


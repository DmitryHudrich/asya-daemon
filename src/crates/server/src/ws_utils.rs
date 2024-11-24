use std::sync::Arc;

use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
use async_trait::async_trait;
use features::workers::Observer;
use futures_util::StreamExt;
use tokio::sync::RwLock;

use crate::routing::route_ws;

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            if let Ok(AggregatedMessage::Text(text)) = msg {
                route_ws(&mut session, text.to_string()).await;
            }
        }
    });
    Ok(res)
}

pub async fn wsevents_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, _) = actix_ws::handle(&req, stream)?;

    let worker = features::workers::get_actionworker().await;
    let s = session.to_owned();
    worker
        .subscribe(Box::new(PrintObserver {
            session: Arc::new(RwLock::new(s)),
        }))
        .await;
    Ok(res)
}

pub struct PrintObserver {
    session: Arc<RwLock<Session>>,
}

#[async_trait]
impl Observer<String> for PrintObserver {
    async fn update(&self, phrase: &String) {
        let mut session = self.session.write().await;
        let res = session
            .text(serde_json::to_string(phrase).unwrap().to_string())
            .await;
        match res {
            Ok(_) => {}
            Err(_) => return,
        };
    }
}

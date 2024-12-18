use std::sync::Arc;

use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
use futures_util::StreamExt;
use log::{info, warn};
use shared::event_system;
use tokio::task;
use usecases::AsyaResponse;

use crate::{requests::Requests, responses::Responses};

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    handle_message(&mut session, text.to_string()).await;
                }
                Ok(AggregatedMessage::Close(_reason)) => {
                    info!("Closing connection");
                    event_system::unsubscribe_all().await;
                }
                _ => (),
            }
        }
    });

    Ok(res)
}

const DEFAULT_EXPECT_MSG: &str = "The Responses enum should be able to be converted into JSON";

async fn handle_message(session: &mut Session, input: String) {
    let request = serde_json::from_str::<Requests>(&input);
    match request {
        Ok(request) => {
            handle_request(request, session).await;
        }
        Err(err) => {
            handle_error(err, session).await;
        }
    }
}

async fn handle_error(err: serde_json::Error, session: &mut Session) {
    warn!("Error parsing request: {:?}", err);

    let response = Responses::Base {
        is_err: true,
        message: err.to_string(),
    };
    session
        .text(
            serde_json::to_string(&response)
                .expect(DEFAULT_EXPECT_MSG)
                .to_string(),
        )
        .await
        .unwrap();
}

async fn handle_request(request: Requests, session: &mut Session) {
    let Requests::General { action } = request;
    event_system::subscribe_once(
        {
            let session = session.clone(); // should be a clone of the session?
            move |event: Arc<AsyaResponse>| {
                let mut session = session.clone();
                task::spawn(async move {
                    let response = Responses::Base {
                        is_err: false,
                        message: event.to_string(),
                    };

                    session
                        .text(
                            serde_json::to_string(&response)
                                .expect(DEFAULT_EXPECT_MSG)
                                .to_string(),
                        )
                        .await
                        .unwrap(); // here should be handler for disconnect. asya panicks without it
                                   // after few seconds after disconnection
                })
            }
        }
    )
    .await;
    usecases::dispatch_usecase(action, "".to_string()).await;
}

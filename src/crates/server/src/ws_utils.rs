use std::sync::Arc;

use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
use async_trait::async_trait;
use features::{services::commands::music, workers::Observer};
use futures_util::StreamExt;
use log::warn;
use tokio::sync::RwLock;

use crate::{
    requests::{MusicAction, Requests},
    responses::Responses,
};

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            if let Ok(AggregatedMessage::Text(text)) = msg {
                handle_message(&mut session, text.to_string()).await;
            }
        }
    });

    Ok(res)
}

pub async fn ws_events_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, _) = actix_ws::handle(&req, stream)?;
    let worker = features::workers::get_actionworker().await;

    worker
        .subscribe(Box::new(PrintObserver {
            session: Arc::new(RwLock::new(session)),
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
        let _ = self.session.write().await.text(phrase.to_owned()).await;
    }
}

async fn handle_message(session: &mut Session, input: String) {
    let request = extract_request(input);

    if let Requests::Music { action } = request {
        handle_music(action, session).await;
    }
}

async fn handle_music(action: MusicAction, session: &mut Session) {
    const DEFAULT_EXPECT_MSG: &str =
        "The Responses enum should be able to be converted into JSON";

    match action {
        MusicAction::PlayPause => {
            music::play_pause();

            let response = Responses::Base {
                is_err: false,
                message: "Toggled between play/pause in the player".to_string(),
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
        MusicAction::GetStatus => {
            let status = music::get_status();

            let response = Responses::Base {
                is_err: false,
                message: status.to_string(),
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
        MusicAction::Next => todo!(),
        MusicAction::Previous => todo!(),
    }
}

fn extract_request(input: String) -> Requests {
    serde_json::from_str::<Requests>(&input).unwrap_or_else(|err| {
        warn!("{:?}", err);
        Requests::Empty
    })
}

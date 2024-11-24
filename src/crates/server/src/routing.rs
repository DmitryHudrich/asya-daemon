use actix_web::{dev::ServiceFactory, middleware, web, App, Error};
use actix_ws::Session;
use features::services::commands::music;
use log::warn;

use crate::{
    requests::{MusicAction, Requests},
    responses::Responses,
    ws_utils,
};

pub fn route_all() -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
        .route("/hey", web::get().to(|| async { "bebra" }))
        .route("/sex", web::get().to(|| async { "не было" }))
        .route("/ws", web::get().to(ws_utils::ws_handler))
}

pub async fn route_ws(session: &mut Session, input: String) {
    let request = extract_request(input);
    if let Requests::Music { action } = request {
        handle_music(action, session).await;
    }
}

async fn handle_music(action: MusicAction, session: &mut Session) {
    match action {
        MusicAction::PlayPause => {
            music::play_pause();
            let response = Responses::Base {
                is_err: false,
                message: "i don't know what i should write here.".to_string(),
            };
            session
                .text(serde_json::to_string(&response).unwrap().to_string())
                .await
                .unwrap();
        }
        MusicAction::GetStatus => {
            let status = music::get_status();
            let response = Responses::Base {
                is_err: false,
                message: format!("{:?}", status),
            };
            session
                .text(serde_json::to_string(&response).unwrap().to_string())
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

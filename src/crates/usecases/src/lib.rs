use crate::usecases::Usecases;
use log::*;
use serde::Serialize;
use services::llm_api;
use shared::event_system;
use shared::plugin_system::ReadableRequest;
use std::sync::Arc;
use tokio::task;

pub mod scenarios;
pub mod shared_workers;
mod tools;
pub mod usecases;

fn process_response(llm_response: &str) -> Result<Usecases, Box<dyn std::error::Error>> {
    let llm_response = llm_response.replace("`json", "");
    let llm_response = llm_response.replace("`", "");
    let usecase = serde_json::from_str::<Usecases>(&llm_response.clone())?;
    Ok(usecase)
}

pub async fn subscribe_for_plugins() {
    event_system::subscribe_once({
        move |event: Arc<ReadableRequest>| {
            task::spawn(async move {
                dispatch_by_user_message(event.0.to_string()).await;
            })
        }
    })
    .await
}

pub async fn dispatch_by_user_message(message: String) {
    let schema = schemars::schema_for!(Usecases);

    let req = format!(
        "
            Generate json representation of command from this user input: {}
            by this json schema fo available commands: {}

            SEND ME ONLY GENERATED JSON

        ",
        message,
        serde_json::to_string_pretty(&schema).unwrap()
    );

    let llm_response = llm_api::send_request(req).await;

    if llm_response.is_err() {
        warn!("Error sending request to LLM: {:?}", llm_response.err());
        return;
    }
    let llm_response = llm_response.unwrap();
    debug!("LLM RESPONSE: {}", llm_response);
    let usecase = process_response(&llm_response);
    if let Err(err) = usecase {
        warn!("Error parsing response from LLM: {:?}", err);
        return;
    }
    let usecase = usecase.unwrap();
    usecase.execute(message).await;
}

// general purpose events

/// General response event. Use it to send responses to the client.
/// How event works see [`shared::event_system`].
#[derive(Debug, parse_display::Display, Serialize)]
#[serde(tag = "asyaResponse")]
#[serde(rename_all = "camelCase")]
pub enum AsyaResponse {
    /// Success response with message from Asya.
    ///
    /// # Arguments
    ///     * `message` - human readable message from Asya, e.g.
    ///         "I've turned off the music. Don't listen this shit anymore."
    ///
    /// # Example
    ///
    /// ```
    /// event_system::publish(AsyaResponse::Ok {
    ///     message: "Hi, Vitaliy! I heard that u like thinkpads? Me too!"
    /// }
    ///
    /// ```
    #[display("{message}")]
    Ok { message: String },
}

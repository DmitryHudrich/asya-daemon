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
    let llm_response = llm_response.replace("`", "");
    let temp_value = serde_json::from_str::<serde_json::Value>(&llm_response)?;
    let command = temp_value
        .pointer("/command/action")
        .ok_or("Command not found")?;
    let usecase = serde_json::from_value::<Usecases>(command.clone())?;
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
    // should be autogenerated.
    let stringified_usecases = Usecases::stringify();
    let stringified_usecases = normalize(stringified_usecases);
    println!("usecases: {} \nend", stringified_usecases);

    let req = format!(
        "
            Generate json. your json will be used for parsing, so don't use any markdown.
            use json templates:
            ```
                {{
                    \"command\": {{
                        \"action\": \"turnOffMusic\" 
                    }}
                }}
            ```
            ```
                {{
                    \"command\": {{
                        \"action\" {{
                            \"open\": {{ 
                                \"appKind\": \"terminal\" 
                            }}
                        }}
                    }}
                }}
            ```
            Generate for the following query:
            Here is commands which are available: {}. 
            Choose command that mostly looks like this description: {}? 
            SEND ME ONLY COMMAND AND POSSIBLE PARAMETERS
        ",
        stringified_usecases, message
    );
    println!("REQ: {}", req);
    let llm_response = llm_api::send_request(req).await;
    println!("llm: {:?}", llm_response);

    if llm_response.is_err() {
        warn!("Error sending request to LLM: {:?}", llm_response.err());
        return;
    }
    let llm_response = llm_response.unwrap();

    println!("LLM: {}", llm_response);
    let usecase = process_response(&llm_response);
    if let Err(err) = usecase {
        warn!("Error parsing response from LLM: {:?}", err);
        return;
    }
    let usecase = usecase.unwrap();
    usecase.execute(message).await;
}

fn normalize(stringified_usecases: &str) -> String {
    let lines: Vec<&str> = stringified_usecases.lines().collect();

    let updated_lines = &lines[1..lines.len() - 1];
    if lines.len() > 2 {
        for line in updated_lines {
            println!("{}", line);
        }
    }
    let x = String::from_iter(updated_lines.iter().map(|el| el.to_string() + "\n"));
    to_camel_case(x.as_str())
}

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut words = Vec::from_iter(input.split_whitespace().map(|s| s.to_string()));
    for word in &mut words {
        word.replace_range(
            0..1,
            word.chars()
                .next()
                .unwrap()
                .to_string()
                .to_lowercase()
                .as_str(),
        )
    }
    dbg!(&words);
    result.extend(words.iter().map(|word| word.to_owned() + "\n"));
    result
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

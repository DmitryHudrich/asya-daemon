use rand::seq::SliceRandom;

use crate::AsyaResponse;

static PHRASES: &[&str] = &["Hi!", "How r u?", "ну шо ти епта"];

// for future use
// #[derive(Debug)]
// enum RandomPhraseEvent {
//     Phrase(String),
// }

pub async fn run() {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    // заебует
    // loop {
        interval.tick().await;
        let phrase = PHRASES.choose(&mut rand::thread_rng()).unwrap();
        crate::publish(AsyaResponse::Ok { message: phrase.to_string() }).await;
    // }
}

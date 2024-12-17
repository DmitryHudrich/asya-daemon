use std::collections::HashMap;

use services::{lexicon::Lexicon, llm_api};
use shared::llm;

#[derive(Debug, Default)]
pub struct PromptBuilder {
    varibles: HashMap<String, String>,
    fallback_phrase: Lexicon,
    prompt_path: String,
}

impl PromptBuilder {
    pub fn new() -> Self {
        PromptBuilder::default()
    }

    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.prompt_path = path.to_string();
        self
    }

    pub fn set_fallback_phrase(&mut self, phrase: Lexicon) -> &mut Self {
        self.fallback_phrase = phrase;
        self
    }

    pub fn set_variable(&mut self, key: &str, value: &str) -> &mut Self {
        self.varibles.insert(key.to_string(), value.to_string());
        self
    }

    pub async fn get_result(&self) -> String {
        let mut prompt = llm::get_prompt(self.prompt_path.as_str());
        for (key, value) in &self.varibles {
            prompt = prompt.replace(key, value);
        }
        let response = llm_api::send_request(prompt).await;
        response.unwrap_or(Lexicon::MusicResume.describe().to_string())
    }
}


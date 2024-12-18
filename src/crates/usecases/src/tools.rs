use std::collections::HashMap;

use services::{lexicon::Lexicon, llm_api};
use shared::llm;

/// Builder for creating a human readable response from Asya.
///
/// # Example:
///
/// ```rust
/// let res = PromptBuilder::new()
///     .set_path("/telegram/music/pause")
///     .set_variable("{command}", executed_command.as_str())
///     .set_fallback_phrase(Lexicon::MusicPause)
///     .get_result()
///     .await;
/// ```
#[derive(Debug, Default)]
pub struct PromptBuilder {
    varibles: HashMap<String, String>,
    fallback_phrase: Lexicon,
    prompt_path: String,
}

impl PromptBuilder {
    /// Creates a new `PromptBuilder` with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = PromptBuilder::new();
    /// ```
    pub fn new() -> Self {
        PromptBuilder::default()
    }

    /// Sets the path pointer for the prompt.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the prompt.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = PromptBuilder::new().set_path("/path/to/prompt/in/ai-prompts.yaml");
    /// ```
    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.prompt_path = path.to_string();
        self
    }

    /// Sets the fallback phrase to be used if the prompt fails.
    ///
    /// # Arguments
    ///
    /// * `phrase` - A `Lexicon` instance representing the fallback phrase.
    ///
    /// # Examples
    ///
    /// ```
    /// let lexicon = Lexicon::Error;
    /// let builder = PromptBuilder::new().set_fallback_phrase(lexicon);
    /// ```
    pub fn set_fallback_phrase(&mut self, phrase: Lexicon) -> &mut Self {
        self.fallback_phrase = phrase;
        self
    }

    /// Sets a variable to be used in the prompt.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key of the variable.
    /// * `value` - A string slice that holds the value of the variable.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = PromptBuilder::new().set_variable("{name}", "value");
    /// ```
    pub fn set_variable(&mut self, key: &str, value: &str) -> &mut Self {
        self.varibles.insert(key.to_string(), value.to_string());
        self
    }

    /// Asynchronously gets the result of the prompt.
    ///
    /// This method sends a request to the LLM API with the constructed prompt
    /// and returns the response. If the request fails, it returns the fallback phrase.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = builder.get_result().await;
    /// ```
    ///
    /// # Returns
    ///
    /// A `String` containing the result of the prompt or the fallback phrase.
    pub async fn get_result(&self) -> String {
        let mut prompt = llm::get_prompt(self.prompt_path.as_str());
        for (key, value) in &self.varibles {
            prompt = prompt.replace(key, value);
        }
        let response = llm_api::send_request(prompt).await;
        response.unwrap_or(self.fallback_phrase.describe().to_string())
    }
}

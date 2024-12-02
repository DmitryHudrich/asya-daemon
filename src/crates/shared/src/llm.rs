use crate::serde_extensions::get_yaml_value;

const PROMPT_CONFIG: &str = "ai-prompts.yaml";
pub fn get_prompt(path: &str) -> String {
    let err_msg = "The prompt config must be possible to load";
    let content = std::fs::read_to_string(PROMPT_CONFIG).expect(err_msg);
    get_yaml_value(&content, path).expect(err_msg)
}

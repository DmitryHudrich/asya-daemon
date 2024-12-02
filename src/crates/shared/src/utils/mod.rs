pub mod file_utils;
pub mod shell_utils;

pub mod llm_utils {
    use super::file_utils;

    const PROMPT_CONFIG: &str = "ai-prompts.yaml";
    pub fn get_prompt(path: &str) -> String {
        let err_msg = "Error while load prompt config.";
        let content = &std::fs::read_to_string(PROMPT_CONFIG).expect(err_msg);
        file_utils::get_yaml_value(content, path).expect(err_msg)
    }
}

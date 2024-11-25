pub mod file_utils;
pub mod shell_utils;

pub mod llm_utils {
    use super::file_utils;

    const PROMPT_CONFIG: &str = "ai-prompts.yaml";
    pub fn get_prompt(path: &str) -> String {
        let err_msg = "Error while load prompt config.";
        let load_file = &file_utils::load_file(PROMPT_CONFIG).expect(err_msg);
        let content = load_file.as_str();
        file_utils::get_yaml_value(content, path).expect(err_msg)
    }
}

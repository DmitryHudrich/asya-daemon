mod fetch;

pub fn fetchservice_handler(
    params: Vec<(String, String)>,
) -> std::collections::HashMap<String, serde_json::Value> {
    fetch::parse(params)
}

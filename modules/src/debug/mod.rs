pub(crate) mod workers;

pub async fn init_module() -> Result<String, String> {
    workers::get_runner().await;
    Ok("Ok".to_owned())
}

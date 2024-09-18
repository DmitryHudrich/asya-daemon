mod debug;
mod telegram;

pub async fn init_modules() {
    debug::init_module().await;
}

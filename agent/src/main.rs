use env_logger;

mod core;

pub mod file;
pub mod environment;
pub mod signature;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

#[tokio::main]
async fn main() {
    setup();

    let mut core: core::Core = core::Core::new().await;

    core.run().await;
}

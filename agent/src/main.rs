mod config;
mod core;
mod security;
mod storage;
mod utils;

use log::info;

#[tokio::main]
async fn main() {
    config::logger::Logger::init().expect("Failed to initialize logger");
 
    info!("Starting Glacier application");
    let mut core = core::Core::new().await;
    core.run().await;
    info!("Glacier application completed");
}

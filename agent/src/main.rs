mod core;

pub mod file;
pub mod environment;
pub mod signature;

use log::LevelFilter;
use chrono::Local;
use fern::Dispatch;
use std::fs::create_dir_all;

fn setup_logger() -> Result<(), fern::InitError> {
    create_dir_all("logs")?;
    Dispatch::new()
        .chain(std::io::stdout())
        .chain(fern::log_file("logs/agent.log")?)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    setup_logger().expect("Failed to initialize logger");

    let mut core: core::Core = core::Core::new().await;

    core.run().await;
}

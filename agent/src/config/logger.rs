use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use std::fs::create_dir_all;
use crate::utils::constants::LOG_DIRECTORY;

pub struct Logger;

impl Logger {
    pub fn init() -> Result<(), fern::InitError> {
        create_dir_all(LOG_DIRECTORY)?;

        Dispatch::new()
            .chain(std::io::stdout())
            .chain(fern::log_file(format!("{}/glacier.log", LOG_DIRECTORY))?)
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
}

use log::info;
use std::io;

pub struct SecurityHandler {
    encryption_key: String,
}

impl SecurityHandler {
    pub fn new(encryption_key: &str) -> Self {
        info!("Initializing security handler");
        Self {
            encryption_key: encryption_key.to_string(),
        }
    }

    pub fn encrypt(&self, content: &str) -> io::Result<String> {
        info!("Encrypting content");
        Ok(content.to_string())
    }

    pub fn decrypt(&self, content: &str) -> io::Result<String> {
        info!("Decrypting content");
        Ok(content.to_string())
    }
}
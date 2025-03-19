use std::fs;
use std::io;
use log::info;

pub struct File {
    pub path: String,
}

impl File {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

pub struct FileHandler {
    pub storage_dir: String,
}

impl FileHandler {
    pub fn new(storage_dir: &str) -> Self {
        info!("Storage directory set: {}", storage_dir);
        Self { storage_dir: storage_dir.to_string() }
    }

    pub fn prepare_file(&self, file_name: &str) -> String {
        format!("{}/{}", self.storage_dir, file_name)
    }

    pub fn save_file(&self, file_name: &str, content: &str) -> io::Result<String> {
        let file_path = self.prepare_file(file_name);
        info!("Saving file: {}", file_name);
        fs::write(&file_path, content)?;
        Ok(file_path)
    }

    pub fn read_file(&self, file_name: &str) -> io::Result<String> {
        let file_path = self.prepare_file(file_name);
        info!("Reading file: {}", file_name);
        fs::read_to_string(&file_path)
    }
}

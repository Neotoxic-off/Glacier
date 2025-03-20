use std::fs;
use std::io;
use log::info;

pub struct File {
    pub path: String,
    pub name: String
}

impl File {
    pub fn new(path: &str, name: &str) -> Self {
        Self { 
            path: path.to_string(),
            name: name.to_string(),
        }
    }
}

pub struct FileHandler {
    storage_dir: String,
}

impl FileHandler {
    pub fn new(storage_dir: &str) -> Self {
        info!("Storage directory set: {}", storage_dir);
        Self { storage_dir: storage_dir.to_string() }
    }

    pub fn get_storage_dir(&self) -> &str {
        &self.storage_dir
    }

    pub fn prepare_file_path(&self, file_name: &str) -> String {
        format!("{}/{}", self.storage_dir, file_name)
    }

    pub fn create_file(&self, file_name: &str) -> File {
        let path: String = self.prepare_file_path(file_name);
        File::new(&path, file_name)
    }
}

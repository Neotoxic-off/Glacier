use std::fs;
use std::io;
use log::info;

pub struct File {
    path: String,
    name: String,
}

impl File {
    pub fn new(path: &str, name: &str) -> Self {
        Self { 
            path: path.to_string(),
            name: name.to_string(),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
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
        let path = self.prepare_file_path(file_name);
        File::new(&path, file_name)
    }

    pub fn save_file(&self, file_name: &str, content: &str) -> io::Result<File> {
        let file_path = self.prepare_file_path(file_name);
        
        info!("Saving file: {}", file_name);
        fs::write(&file_path, content)?;

        Ok(File::new(&file_path, file_name))
    }

    pub fn read_file(&self, file_name: &str) -> io::Result<String> {
        let file_path = self.prepare_file_path(file_name);

        info!("Reading file: {}", file_name);
        fs::read_to_string(&file_path)
    }
}

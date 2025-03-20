use std::io::{self, Read, Write};
use log::info;
use std::fs::File as FsFile;

pub struct File {
    pub path: String
}

impl File {
    pub fn new(path: &str) -> Self {
        Self { 
            path: path.to_string()
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

        File::new(&path)
    }

    pub fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        let mut file: FsFile = FsFile::open(path)?;
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    pub fn write_file(&self, path: &str, data: &[u8]) -> io::Result<()> {
        let mut file: FsFile = FsFile::create(path)?;
        file.write_all(data)?;

        Ok(())
    }
}

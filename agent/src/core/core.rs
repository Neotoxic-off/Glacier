use std::collections::HashMap;
use std::error::Error;
use std::fs;
use chrono::Local;
use csv::Writer;
use log::{error, info, warn};
use crate::config::environment::Environment;
use crate::security::security::SecurityHandler;
use crate::storage::file_handler::FileHandler;
use crate::storage::signature_handler::SignatureHandler;
use crate::utils::constants::REPORT_DIRECTORY;

pub struct Core {
    file_handler: FileHandler,
    signature_handler: SignatureHandler,
    files_status: HashMap<String, FileStatus>
}

pub struct FileStatus {
    status: String,
    signature: String,
}

impl Core {
    pub async fn new() -> Self {
        let env: Environment = Environment::new().expect("Failed to load environment variables");
        let file_handler: FileHandler = FileHandler::new(&env.storage_directory);
        let security_handler: SecurityHandler = SecurityHandler::new(&env.encryption_key);
        let signature_handler: SignatureHandler = SignatureHandler::new(
            &env.database_url,
            &env.database_name
        ).await;

        Self {
            file_handler,
            signature_handler,
            files_status: HashMap::new()
        }
    }

    pub async fn run(&mut self) {
        info!("❄️ Glacier initialized and ready");
        self.verify_files().await;
        self.display_files_status();
        if let Err(e) = self.save_report() {
            error!("Failed to save report: {}", e);
        }
    }

    fn save_report(&self) -> Result<(), Box<dyn Error>> {
        let now: chrono::DateTime<Local> = Local::now();
        let date: String = now.format("%Y-%m-%d").to_string();
        let hour: String = now.format("%H-%M-%S").to_string();
        let folder_path: String = format!("{}/{}", REPORT_DIRECTORY, date);
        let file_path: String = format!("{}/{}.csv", folder_path, hour);

        fs::create_dir_all(&folder_path)?;

        let mut wtr: Writer<fs::File> = Writer::from_path(&file_path)?;

        wtr.write_record(&["file", "status", "signature"])?;

        for (key, file_status) in &self.files_status {
            wtr.write_record(&[key, &file_status.status, &file_status.signature])?;
        }

        wtr.flush()?;
        info!("Report saved to: {}", file_path);

        Ok(())
    }

    async fn verify_files(&mut self) {
        let files: fs::ReadDir = match fs::read_dir(&self.file_handler.get_storage_dir()) {
            Ok(files) => files,
            Err(e) => {
                error!("Failed to read storage directory: {}", e);
                return;
            }
        };

        for file in files.flatten() {
            let file_name: String = file.file_name().to_string_lossy().to_string();
            let path: String = self.file_handler.prepare_file_path(&file_name);
            self.compare_signatures(path).await;
        }
    }

    async fn compare_signatures(&mut self, file_path: String) {
        let file_name = match file_path.split('/').last() {
            Some(name) => name,
            None => {
                error!("Invalid file path: {}", file_path);
                return;
            }
        };
        let signature_data = self.signature_handler.load_signature_with_leaves(file_name).await;
        
        match signature_data {
            Some((original_signature, original_leaves, chunk_positions)) => {
                match self.signature_handler.check_broken_chunks(&file_path, &original_signature, &original_leaves, Some(&chunk_positions)) {
                    Ok(corrupted_chunks) if corrupted_chunks.is_empty() => {
                        info!("File '{}' integrity check passed", file_path);
                        self.files_status.insert(file_path, FileStatus {
                            status: "valid".to_string(),
                            signature: original_signature
                        });
                    }
                    Ok(corrupted_chunks) => {
                        error!(
                            "File '{}' has corrupted chunks: {:?}",
                            file_path, corrupted_chunks
                        );
                        self.files_status.insert(file_path, FileStatus {
                            status: "corrupted".to_string(),
                            signature: original_signature
                        });
                    }
                    Err(e) => {
                        error!("File integrity check failed for '{}': {}", file_path, e);
                        self.files_status.insert(file_path, FileStatus {
                            status: "error".to_string(),
                            signature: original_signature
                        });
                    }
                }
            },
            None => {
                let (generated_signature, generated_leaves, chunk_positions) = 
                    self.signature_handler.generate_signature_with_leaves(&file_path);
                
                if generated_signature.is_empty() {
                    error!("Failed to generate signature for {}", file_path);
                    return;
                }
    
                if let Err(e) = self.signature_handler.save_signature(
                    file_name, 
                    &generated_signature, 
                    &generated_leaves,
                    &chunk_positions
                ).await {
                    error!("Failed to save signature for {}: {}", file_path, e);
                    return;
                } else {
                    info!("Saved signature with {} chunks for {}", generated_leaves.len(), file_path);
                }
    
                self.files_status.insert(file_path, FileStatus {
                    status: "initialized".to_string(),
                    signature: generated_signature
                });
            }
        }
    }

    fn display_files_status(&self) {
        for (file, file_status) in &self.files_status {
            match file_status.status.as_str() {
                "initialized" => warn!("File '{}' saved and signature generated.", file),
                "valid" => info!("File '{}' integrity valid.", file),
                "corrupted" => error!("File '{}' integrity check invalid.", file),
                "error" => error!("File '{}' integrity check error.", file),
                _ => {}
            }
        }
    }
}

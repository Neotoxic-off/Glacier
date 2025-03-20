use std::collections::HashMap;
use std::error::Error;
use std::fs;
use chrono::Local;
use csv::Writer;
use log::{error, info, warn};

use crate::config::environment::Environment;
use crate::security;
use crate::security::security::SecurityHandler;
use crate::storage::file_handler::{File, FileHandler};
use crate::storage::signature_handler::SignatureHandler;
use crate::utils::constants::{REPORT_DIRECTORY, GLACIER_DIRECTORY};

pub struct Core {
    file_handler: FileHandler,
    signature_handler: SignatureHandler,
    files_status: HashMap<String, FileStatus>,
    security_handler: SecurityHandler
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
            files_status: HashMap::new(),
            security_handler
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
            let file: File = self.file_handler.create_file(&file_name);
            let file_content: Vec<u8> = match self.file_handler.read_file(&file.path) {
                Ok(buff) => buff,
                Err(e) => {
                    error!("Failed to read file {}: {}", file_name, e);
                    Vec::new()
                }
            };
            
            let file_registered: bool = self.signature_handler.is_in_catalog(&file_name).await;
            let current_signature: String;

            if file_registered == false {
                match self.signature_handler.save_to_catalog(&file_name).await {
                    Ok(_) => {},
                    Err(_) => {}
                }
                match self.file_handler.write_file(&format!("{}/{}.enc", GLACIER_DIRECTORY, file_name), &self.security_handler.encrypt(&file_content)) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            }

            current_signature = self.signature_handler.generate_signature(&file.path);

            self.compare_signatures(file_name, current_signature).await;
        }
    }

    async fn compare_signatures(&mut self, file_name: String, current_signature: String) {
        match self.signature_handler.load_signature(&file_name).await {
            Some(signature) if signature == current_signature => {
                self.files_status.insert(file_name, FileStatus {
                    status: "valid".to_string(),
                    signature: current_signature,
                });
            }
            Some(_) => {
                self.files_status.insert(file_name, FileStatus {
                    status: "corrupted".to_string(),
                    signature: current_signature,
                });
            }
            None => {
                if let Err(e) = self.signature_handler.save_signature(&file_name, &current_signature).await {
                    error!("Failed to save signature for {}: {}", file_name, e);
                    return;
                }
                
                self.files_status.insert(file_name, FileStatus {
                    status: "initialized".to_string(),
                    signature: current_signature,
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
                _ => {}
            }
        }
    }
}

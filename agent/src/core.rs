use std::fs;
use log::{info, error, warn};
use std::collections::HashMap;

use crate::environment;
use crate::file;
use crate::signature;

pub struct Core {
    pub file_handler: file::FileHandler,
    pub signature_handler: signature::SignatureHandler,
    pub files_status: HashMap<String, String>,
}

impl Core {
    pub async fn new() -> Self {
        let env = environment::Environment::new();
        let file_handler = file::FileHandler::new(&env.storage_directory);
        let signature_handler = signature::SignatureHandler::new(
            &env.database_url,
            &env.database_name,
            &env.database_collection,
        ).await;

        Self {
            file_handler,
            signature_handler,
            files_status: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        info!("❄️ Glacier initialized and ready.");
        self.verify_files().await;
        self.display_files_status();
    }

    async fn verify_files(&mut self) {
        let files = fs::read_dir(&self.file_handler.storage_dir)
            .expect("Failed to list directory");

        for file in files.flatten() {
            let file_name = file.file_name().to_string_lossy().to_string();
            let file_path = self.file_handler.prepare_file(&file_name);
            let current_signature = self.signature_handler.generate_signature(&file_path);
            self.compare_signatures(file_name, current_signature).await;
        }
    }

    async fn compare_signatures(&mut self, file_name: String, current_signature: String) {
        match self.signature_handler.load_signature(&file_name).await {
            Some(signature) if signature == current_signature => {
                self.files_status.insert(file_name, "valid".to_string());
            }
            Some(_) => {
                self.files_status.insert(file_name, "corrupted".to_string());
            }
            None => {
                self.signature_handler.save_signature(&file_name, &current_signature).await
                    .expect("Failed to save signature");
                self.files_status.insert(file_name, "initialized".to_string());
            }
        }
    }

    fn display_files_status(&self) {
        for (file, status) in &self.files_status {
            match status.as_str() {
                "initialized" => warn!("File '{}' saved and signature generated.", file),
                "valid" => info!("File '{}' integrity valid.", file),
                "corrupted" => error!("File '{}' integrity check invalid.", file),
                _ => {}
            }
        }
    }
}

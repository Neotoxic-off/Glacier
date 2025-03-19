use sha2::{Sha256, Digest};
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection};
use std::fs;
use std::io::Read;
use log::{info, error};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Signature {
    file_name: String,
    signature: String,
}

pub struct SignatureHandler {
    pub db_url: String,
    pub collection: Collection<Signature>,
}

impl SignatureHandler {
    pub async fn new(db_url: &str, db_name: &str, collection_name: &str) -> Self {
        info!("Connecting to MongoDB");
        let client_options: ClientOptions = ClientOptions::parse(db_url).await.expect("Failed to parse MongoDB URL");
        let client: Client = Client::with_options(client_options).expect("Failed to connect to MongoDB");
        let database: mongodb::Database = client.database(db_name);
        let collection: Collection<Signature> = database.collection::<Signature>(collection_name);

        info!("Connected to MongoDB for signature storage.");

        Self {
            db_url: db_url.to_string(),
            collection,
        }
    }

    pub fn generate_signature(&self, file_path: &str) -> String {
        info!("Generating signature for file: {}", file_path);
        let mut hasher = Sha256::new();
        let mut file = fs::File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");
        hasher.update(&buffer);
        format!("{:x}", hasher.finalize())
    }

    pub async fn save_signature(&self, file_name: &str, signature: &str) -> Result<()> {
        info!("Saving signature for {}", file_name);
        let signature_doc = Signature {
            file_name: file_name.to_string(),
            signature: signature.to_string(),
        };

        self.collection
            .insert_one(signature_doc)
            .await
            .expect("Failed to save signature");

        Ok(())
    }

    pub async fn load_signature(&self, file_name: &str) -> Option<String> {
        info!("Loading signature for {}", file_name);
        match self.collection.find_one(doc! { "file_name": file_name }).await {
            Ok(Some(doc)) => Some(doc.signature),
            Ok(None) => None,
            Err(e) => {
                error!("Failed to load signature: {:?}", e);
                None
            }
        }
    }
}

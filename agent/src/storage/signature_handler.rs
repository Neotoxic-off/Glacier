use rs_merkle::{Hasher, MerkleTree};
use rs_merkle::algorithms::Sha256 as MerkleHasher;
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection};
use std::fs;
use std::io::Read;
use log::{info, error};
use hex;

use crate::utils::constants::{COLLECTION_NAME_CATALOG, COLLECTION_NAME_SIGNATURES};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Signature {
    file_name: String,
    signature: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    file_name: String
}

pub struct SignatureHandler {
    signatures: Collection<Signature>,
    catalog: Collection<Catalog>
}

impl SignatureHandler {
    pub async fn new(db_url: &str, db_name: &str) -> Self {
        let client_options: ClientOptions = ClientOptions::parse(db_url)
            .await
            .expect("Failed to parse MongoDB URL");
            
        let client: Client = Client::with_options(client_options)
            .expect("Failed to connect to MongoDB");
            
        let database: mongodb::Database = client.database(db_name);
        let signatures: Collection<Signature> = database.collection::<Signature>(COLLECTION_NAME_SIGNATURES);
        let catalog: Collection<Catalog> = database.collection::<Catalog>(COLLECTION_NAME_CATALOG);

        info!("Connected to MongoDB for signature storage.");

        Self {
            signatures,
            catalog
        }
    }

    pub fn generate_signature(&self, file_path: &str) -> String {
        let chunk_size: usize = 1024;
        
        let file: fs::File = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open file {}: {}", file_path, e);
                return String::new();
            }
        };
        
        let mut buffer: Vec<u8> = Vec::new();
        let mut file_reader: std::io::BufReader<fs::File> = std::io::BufReader::new(file);

        info!("Generating Merkle tree hash for file: {}", file_path);
        
        if let Err(e) = file_reader.read_to_end(&mut buffer) {
            error!("Failed to read file {}: {}", file_path, e);
            return String::new();
        }

        let leaves: Vec<[u8; 32]> = buffer
            .chunks(chunk_size)
            .map(|chunk| MerkleHasher::hash(chunk))
            .collect();

        let merkle_tree: MerkleTree<MerkleHasher> = MerkleTree::<MerkleHasher>::from_leaves(&leaves);
        let root: [u8; 32] = match merkle_tree.root() {
            Some(root) => root,
            None => {
                error!("Merkle tree is empty — failed to calculate root");
                return String::new();
            }
        };

        hex::encode(root)
    }

    pub async fn save_signature(&self, file_name: &str, signature: &str) -> Result<()> {
        let signature_doc = Signature {
            file_name: file_name.to_string(),
            signature: signature.to_string(),
        };

        info!("Saving signature for {}", file_name);
        self.signatures
            .insert_one(signature_doc)
            .await?;

        Ok(())
    }

    pub async fn save_to_catalog(&self, file_name: &str) -> Result<()> {
        let catalog_doc: Catalog = Catalog {
            file_name: file_name.to_string()
        };

        info!("Saving file '{}' to catalog", file_name);
        self.catalog
            .insert_one(catalog_doc)
            .await?;

        Ok(())
    }

    pub async fn is_in_catalog(&self, file_name: &str) -> bool {
        let query: mongodb::bson::Document = doc! { "file_name": file_name };

        info!("Loading signature for {}", file_name);
        match self.catalog.find_one(query).await {
            Ok(Some(doc)) => {
                info!("File '{}' found in catalog", doc.file_name);
                true
            }
            Ok(None) => {
                info!("File '{}' not found in catalog", file_name);
                false
            }
            Err(e) => {
                error!("Failed to load signature: {:?}", e);
                false
            }
        }
    }

    pub async fn load_signature(&self, file_name: &str) -> Option<String> {
        let query: mongodb::bson::Document = doc! { "file_name": file_name };

        info!("Loading signature for {}", file_name);
        match self.signatures.find_one(query).await {
            Ok(Some(doc)) => Some(doc.signature),
            Ok(None) => {
                info!("No signature found for {}", file_name);
                None
            }
            Err(e) => {
                error!("Failed to load signature: {:?}", e);
                None
            }
        }
    }
}

use rs_merkle::{Hasher, MerkleTree};
use rs_merkle::algorithms::Sha256 as MerkleHasher;
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection};
use std::fs;
use std::io::Read;
use log::{info, error};
use hex;

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
        let client_options: ClientOptions = ClientOptions::parse(db_url)
            .await
            .expect("Failed to parse MongoDB URL");
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
        let chunk_size: usize = 1024;
        let mut file: fs::File = fs::File::open(file_path).expect("Failed to open file");
        let mut buffer: Vec<u8> = Vec::new();

        info!("Generating Merkle tree hash for file: {}", file_path);
        file.read_to_end(&mut buffer).expect("Failed to read file");

        let leaves: Vec<[u8; 32]> = buffer
            .chunks(chunk_size)
            .map(|chunk| MerkleHasher::hash(chunk))
            .collect();

        let merkle_tree: MerkleTree<MerkleHasher> = MerkleTree::<MerkleHasher>::from_leaves(&leaves);
        let root = match merkle_tree.root() {
            Some(root) => root,
            None => {
                error!("Merkle tree is empty — failed to calculate root");
                return String::new();
            }
        };

        hex::encode(root)
    }

    pub async fn save_signature(&self, file_name: &str, signature: &str) -> Result<()> {
        let signature_doc: Signature = Signature {
            file_name: file_name.to_string(),
            signature: signature.to_string(),
        };

        info!("Saving signature for {}", file_name);
        self.collection
            .insert_one(signature_doc)
            .await
            .expect("Failed to save signature");

        Ok(())
    }

    pub async fn load_signature(&self, file_name: &str) -> Option<String> {
        let query: mongodb::bson::Document = doc! { "file_name": file_name };

        info!("Loading signature for {}", file_name);
        match self.collection.find_one(query).await {
            Ok(Some(doc)) => Some(doc.signature),
            Ok(None) => None,
            Err(e) => {
                error!("Failed to load signature: {:?}", e);
                None
            }
        }
    }
}
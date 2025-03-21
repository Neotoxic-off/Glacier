use rs_merkle::{Hasher, MerkleTree};
use rs_merkle::algorithms::Sha256 as MerkleHasher;
use mongodb::{bson::doc, error::Result, options::ClientOptions, Client, Collection};
use std::fs;
use std::io::{Read, BufReader};
use log::{info, error, warn};
use hex;

use crate::utils::constants::{
    COLLECTION_NAME_CATALOG,
    COLLECTION_NAME_SIGNATURES,
    CDC_WINDOW_SIZE,
    CDC_AVERAGE_CHUNK_SIZE,
    CDC_MASK
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Signature {
    file_name: String,
    signature: String,
    leaves: Vec<String>,
    chunk_positions: Vec<usize>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    file_name: String
}

pub struct SignatureHandler {
    signatures: Collection<Signature>
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

        info!("Connected to storage");

        Self {
            signatures
        }
    }

    fn rolling_hash(buffer: &[u8], window_size: usize) -> Vec<u32> {
        if buffer.len() < window_size {
            return vec![0];
        }
        let mut hashes = Vec::with_capacity(buffer.len() - window_size + 1);
        let mut hash: u32 = 0;
        for i in 0..window_size {
            hash = hash.wrapping_add(buffer[i] as u32);
        }
        hashes.push(hash);
        for i in window_size..buffer.len() {
            hash = hash.wrapping_add(buffer[i] as u32);
            hash = hash.wrapping_sub(buffer[i - window_size] as u32);
            hashes.push(hash);
        }
        
        hashes
    }
    fn find_chunk_boundaries(&self, buffer: &[u8]) -> Vec<usize> {
        let hashes = Self::rolling_hash(buffer, CDC_WINDOW_SIZE);
        let mut boundaries = Vec::new();
        boundaries.push(0);
        
        let mut current_chunk_size = 0;
        let min_chunk_size = CDC_AVERAGE_CHUNK_SIZE / 4;
        let max_chunk_size = CDC_AVERAGE_CHUNK_SIZE * 4;
        
        for (i, hash) in hashes.iter().enumerate() {
            current_chunk_size += 1;
            
            if current_chunk_size >= min_chunk_size {
                if (hash & CDC_MASK) == 0 || current_chunk_size >= max_chunk_size {
                    let boundary = i + CDC_WINDOW_SIZE;
                    if boundary < buffer.len() {
                        boundaries.push(boundary);
                        current_chunk_size = 0;
                    }
                }
            }
        }
        if boundaries.last() != Some(&buffer.len()) {
            boundaries.push(buffer.len());
        }
        
        boundaries
    }

    fn get_chunks<'a>(&self, buffer: &'a [u8], boundaries: &[usize]) -> Vec<&'a [u8]> {
        let mut chunks = Vec::new();
        
        for i in 0..boundaries.len() - 1 {
            let start = boundaries[i];
            let end = boundaries[i + 1];
            chunks.push(&buffer[start..end]);
        }
        
        chunks
    }

    pub fn generate_signature(&self, file_path: &str) -> String {
        let file_result = fs::File::open(file_path);
        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open file {}: {}", file_path, e);
                return String::new();
            }
        };
        
        let mut buffer: Vec<u8> = Vec::new();
        let mut file_reader: BufReader<fs::File> = BufReader::new(file);
    
        info!("Generating content-defined chunking hash for file: {}", file_path);
        
        if let Err(e) = file_reader.read_to_end(&mut buffer) {
            error!("Failed to read file {}: {}", file_path, e);
            return String::new();
        }
    
        if buffer.is_empty() {
            error!("File is empty, cannot generate signature");
            return String::new();
        }

        let boundaries = self.find_chunk_boundaries(&buffer);
        let chunks = self.get_chunks(&buffer, &boundaries);
        let leaves: Vec<[u8; 32]> = chunks
            .iter()
            .map(|chunk| MerkleHasher::hash(chunk))
            .collect();
        
        if leaves.is_empty() {
            error!("No chunks generated, cannot create signature");
            return String::new();
        }
    
        let merkle_tree = MerkleTree::<MerkleHasher>::from_leaves(&leaves);
        let root = match merkle_tree.root() {
            Some(root) => root,
            None => {
                error!("Merkle tree is empty — failed to calculate root");
                return String::new();
            }
        };
    
        hex::encode(root)
    }
    
    pub fn generate_signature_with_leaves(&self, file_path: &str) -> (String, Vec<String>, Vec<usize>) {
        let file_result = fs::File::open(file_path);
        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open file {}: {}", file_path, e);
                return (String::new(), Vec::new(), Vec::new());
            }
        };
        
        let mut buffer = Vec::new();
        let mut file_reader = BufReader::new(file);
        
        if let Err(e) = file_reader.read_to_end(&mut buffer) {
            error!("Failed to read file {}: {}", file_path, e);
            return (String::new(), Vec::new(), Vec::new());
        }
    
        if buffer.is_empty() {
            error!("File is empty, cannot generate signature");
            return (String::new(), Vec::new(), Vec::new());
        }
        let boundaries = self.find_chunk_boundaries(&buffer);
        let chunks = self.get_chunks(&buffer, &boundaries);
        
        let leaves: Vec<[u8; 32]> = chunks
            .iter()
            .map(|chunk| MerkleHasher::hash(chunk))
            .collect();
        
        if leaves.is_empty() {
            error!("No chunks generated, cannot create signature");
            return (String::new(), Vec::new(), Vec::new());
        }
        let leaf_strings: Vec<String> = leaves.iter().map(|leaf| hex::encode(leaf)).collect();
    
        let merkle_tree = MerkleTree::<MerkleHasher>::from_leaves(&leaves);
        let root = match merkle_tree.root() {
            Some(root) => root,
            None => {
                error!("Merkle tree is empty — failed to calculate root");
                return (String::new(), Vec::new(), Vec::new());
            }
        };
    
        info!("Generated {} content-defined chunks for {}", chunks.len(), file_path);
        (hex::encode(root), leaf_strings, boundaries)
    }

    pub fn check_broken_chunks(&self, file_path: &str, original_signature: &str, original_leaves_hex: &[String], chunk_positions: Option<&[usize]>) -> std::result::Result<Vec<usize>, String> {
        let original_root = match hex::decode(original_signature) {
            Ok(bytes) => {
                if bytes.len() != 32 {
                    return Err("Invalid signature length".to_string());
                }
                let mut root = [0u8; 32];
                root.copy_from_slice(&bytes);
                root
            },
            Err(e) => {
                error!("Failed to decode signature: {}", e);
                return Err(format!("Failed to decode signature: {}", e));
            }
        };
        let mut original_leaves: Vec<[u8; 32]> = Vec::new();
        for hex_str in original_leaves_hex {
            match hex::decode(hex_str) {
                Ok(bytes) => {
                    if bytes.len() != 32 {
                        return Err("Invalid leaf hash length".to_string());
                    }
                    let mut leaf = [0u8; 32];
                    leaf.copy_from_slice(&bytes);
                    original_leaves.push(leaf);
                },
                Err(e) => {
                    return Err(format!("Failed to decode leaf hash: {}", e));
                }
            }
        }
        let file: fs::File = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open file {}: {}", file_path, e);
                return Err(format!("Failed to open file: {}", e));
            }
        };
        
        let mut buffer: Vec<u8> = Vec::new();
        let mut file_reader = BufReader::new(file);
        
        if let Err(e) = file_reader.read_to_end(&mut buffer) {
            error!("Failed to read file {}: {}", file_path, e);
            return Err(format!("Failed to read file: {}", e));
        }
    
        if buffer.is_empty() {
            return Err("File is empty".to_string());
        }
    
        let (current_leaves, boundaries) = if let Some(positions) = chunk_positions {
            let chunks = self.get_chunks(&buffer, positions);
            let leaves: Vec<[u8; 32]> = chunks
                .iter()
                .map(|chunk| MerkleHasher::hash(chunk))
                .collect();
            (leaves, positions.to_vec())
        } else {
            let boundaries = self.find_chunk_boundaries(&buffer);
            let chunks = self.get_chunks(&buffer, &boundaries);
            let leaves: Vec<[u8; 32]> = chunks
                .iter()
                .map(|chunk| MerkleHasher::hash(chunk))
                .collect();
            (leaves, boundaries)
        };
        let merkle_tree = MerkleTree::<MerkleHasher>::from_leaves(&current_leaves);
        let current_root = match merkle_tree.root() {
            Some(root) => root,
            None => {
                return Err("Failed to calculate Merkle root".to_string());
            }
        };
        if current_root == original_root {
            return Ok(vec![]);
        }
    
        info!("File signature mismatch detected. Current: {}, Original: {}", 
            hex::encode(current_root), original_signature);
        let mut corrupted_chunks: Vec<usize> = Vec::new();
        let min_len = std::cmp::min(current_leaves.len(), original_leaves.len());
        for i in 0..min_len {
            if current_leaves[i] != original_leaves[i] {
                corrupted_chunks.push(i);
            }
        }
        if current_leaves.len() > original_leaves.len() {
            for i in original_leaves.len()..current_leaves.len() {
                corrupted_chunks.push(i);
            }
        } else if original_leaves.len() > current_leaves.len() {
            for i in current_leaves.len()..original_leaves.len() {
                corrupted_chunks.push(i);
            }
        }
        if corrupted_chunks.is_empty() {
            warn!("File '{}' has changed but specific corrupted chunks couldn't be identified", file_path);
            corrupted_chunks = (0..original_leaves.len()).collect();
        }
    
        Ok(corrupted_chunks)
    }

    pub async fn save_signature(&self, file_name: &str, signature: &str, leaves: &[String], chunk_positions: &[usize]) -> Result<()> {
        let signature_doc = Signature {
            file_name: file_name.to_string(),
            signature: signature.to_string(),
            leaves: leaves.to_vec(),
            chunk_positions: chunk_positions.to_vec(),
        };
    
        info!("Saving signature and {} leaf hashes for {}", leaves.len(), file_name);
        self.signatures
            .insert_one(signature_doc)
            .await?;
    
        Ok(())
    }

    pub async fn load_signature_with_leaves(&self, file_name: &str) -> Option<(String, Vec<String>, Vec<usize>)> {
        let query: mongodb::bson::Document = doc! { "file_name": file_name };
    
        info!("Loading signature and leaf hashes for {}", file_name);
        match self.signatures.find_one(query).await {
            Ok(Some(doc)) => Some((doc.signature, doc.leaves, doc.chunk_positions)),
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

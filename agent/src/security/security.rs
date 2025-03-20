use log::info;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use aes_gcm::aead::generic_array::{GenericArray};
use aes_gcm::aead::consts::U12;

pub struct SecurityHandler {
    encryption_key: Key<Aes256Gcm>,
}

impl SecurityHandler {
    pub fn new(encryption_key: &str) -> Self {
        info!("Initializing security handler");

        let key: &Key<Aes256Gcm> = Key::<Aes256Gcm>::from_slice(encryption_key.as_bytes());

        Self {
            encryption_key: *key,
        }
    }

    pub fn encrypt(&self, content: &str) -> Vec<u8> {
        info!("Encrypting content");

        let nonce: aes_gcm::aead::generic_array::GenericArray<u8, aes_gcm::aead::consts::U12> = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher: Aes256Gcm = Aes256Gcm::new(&self.encryption_key);
        let ciphered_data: Vec<u8> = cipher
            .encrypt(&nonce, content.as_bytes())
            .expect("failed to encrypt");

        let mut encrypted_data: Vec<u8> = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphered_data);

        encrypted_data
    }

    pub fn decrypt(&self, content: Vec<u8>) -> String {
        let (nonce_arr, ciphered_data): (&[u8], &[u8]) = content.split_at(12);
        let nonce: &GenericArray<u8, U12> = Nonce::from_slice(nonce_arr);
        let cipher: Aes256Gcm = Aes256Gcm::new(&self.encryption_key);

        let plaintext: Vec<u8> = cipher
            .decrypt(nonce, ciphered_data)
            .expect("failed to decrypt data");

        String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
    }
}

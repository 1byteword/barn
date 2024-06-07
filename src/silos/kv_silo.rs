use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use rand::RngCore;
use sharks::{Sharks, Share};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::io::{Read, Write};
use std::fs::File;

#[derive(Serialize, Deserialize, Clone)]
pub struct Secret {
    pub iv: Vec<u8>,
    pub encrypted_value: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PersistedSecrets {
    pub secrets: HashMap<String, Secret>,
}

pub struct KVStore {
    secrets: RwLock<HashMap<String, Secret>>,
}

impl KVStore {
    pub fn new() -> Self {
        KVStore {
            secrets: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set_secret(&self, key: String, iv:Vec<u8>, encrypted_value: Vec<u8>) -> std::io::Result<()> {
        // grab write lock
        let mut secrets = self.secrets.write().await;
        secrets.insert(key, Secret { iv, encrypted_value });

        Ok(())
    }

    pub async fn get_secret(&self, key: &str) -> Option<Secret> {
        // grab read lock
        let secrets = self.secrets.read().await;
        secrets.get(key).cloned()
    }

    pub async fn save_to_file_encrypted(&self, filename: &str, master_key: &[u8]) -> std::io::Result<()> {
        let secrets = self.secrets.read().await;
        let persisted_secrets = PersistedSecrets {
            secrets: secrets.clone(),
        };

        let data = serde_json::to_vec(&persisted_secrets)?;
        
        let (iv, encrypted_data) = encrypt_data(master_key, &data);

        let mut file = File::create(filename)?;
        // write IV to file
        file.write_all(&iv)?;
        // write encrypted data to file
        file.write_all(&encrypted_data)?;
        Ok(())
    }

    pub async fn load_from_file_encrypted(&self, filename: &str, master_key: &[u8]) -> std::io::Result<()> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return Ok(()),
        };

        // read IV from file
        // XChaCha20-Poly1305 uses a 24 bit nonce
        let mut iv = vec![0u8; 24];
        file.read_exact(&mut iv)?;

        // read rest of file (encrypted data)
        let mut encrypted_data = Vec::new();
        file.read_to_end(&mut encrypted_data)?;

        // decrypt data
        let data = decrypt_data(master_key, &iv, &encrypted_data);

        // deserialize data
        let persisted_secrets: PersistedSecrets = serde_json::from_slice(&data)?;
        
        // grab write lock
        let mut secrets = self.secrets.write().await;
        *secrets = persisted_secrets.secrets;
        Ok(())
    }
}

pub fn encrypt_data(key: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    // 32 bytes
    let key = Key::from_slice(key);
    let cipher = XChaCha20Poly1305::new(key);

    let mut iv = [0u8; 24];
    OsRng.fill_bytes(&mut iv);
    let nonce = XNonce::from_slice(&iv);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failure!");
    (iv.to_vec(), ciphertext)
}

pub fn decrypt_data(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(key);
    let cipher = XChaCha20Poly1305::new(key);

    let nonce = XNonce::from_slice(iv);

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("decryption failure!");
    plaintext
}

pub fn split_dek(dek: &[u8]) -> Vec<Share> {
    let sharks = Sharks(3);
    let dealer = sharks.dealer(dek);
    dealer.take(5).collect()
}

pub fn reconstruct_dek(shares: Vec<Share>) -> Vec<u8> {
    let sharks = Sharks(3); // minimum 3 shares to recover DEK
    sharks.recover(shares).expect("Failed to recover DEK")
}

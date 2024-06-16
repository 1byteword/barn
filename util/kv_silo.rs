use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::io::{Read, Write};
use std::fs::File;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use rand::RngCore;
use rand::rngs::OsRng;

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

    pub async fn set_secret(&self, key: String, iv: Vec<u8>, encrypted_value: Vec<u8>) -> std::io::Result<()> {
        let mut secrets = self.secrets.write().await;
        secrets.insert(key, Secret { iv, encrypted_value });
        Ok(())
    }

    pub async fn get_secret(&self, key: &str) -> Option<Secret> {
        let secrets = self.secrets.read().await;
        secrets.get(key).cloned()
    }

    pub async fn save_to_file_encrypted(&self, filename: &str, encrypted_data: &[u8], nonce: &[u8]) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(nonce)?;
        file.write_all(encrypted_data)?;
        Ok(())
    }    

    pub async fn load_from_file_encrypted(&self, filename: &str) -> std::io::Result<(Vec<u8>, Vec<u8>)> {
        let mut file = File::open(filename)?;
        let mut nonce = vec![0u8; 24];
        file.read_exact(&mut nonce)?;
        let mut encrypted_data = Vec::new();
        file.read_to_end(&mut encrypted_data)?;
        Ok((nonce, encrypted_data))
    }
}

pub fn encrypt_data(key: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
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
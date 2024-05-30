use sodiumoxide::crypto::secretbox;
use ring::rand::{SecureRandom, SystemRandom};

const KEY_LEN: usize = 32;

pub fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; KEY_LEN];
    let rng = SystemRandom::new();
    rng.fill(&mut key).unwrap();
    key
}

pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let nonce = secretbox::gen_nonce();
    let cipher = secretbox::seal(data, &nonce, &secretbox::Key::from_slice(key).unwrap());
    [nonce.0.to_vec(), cipher].concat()
}

pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() < secretbox::NONCEBYTES {
        return Err("Invalid data");
    }

    let (nonce, cipher) = data.split_at(secretbox::NONCEBYTES);
    secretbox::open(cipher, &secretbox::Nonce::from_slice(nonce).unwrap(), &secretbox::Key::from_slice(key).unwrap())
        .map_err(|_| "Decryption failed")
}
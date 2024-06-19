use actix_web::{web, HttpResponse, Responder, post};
use chacha20poly1305::{XNonce, Key, aead::Aead};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use rand::rngs::OsRng;
use rand::RngCore;

use sodiumoxide::hex;

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct StoreRequest {
    pub key: String,
    pub value: String,
}

#[post("/store")]
async fn store(data: web::Json<StoreRequest>, state: web::Data<AppState>) -> impl Responder {
    let data_dir = "data";
    fs::create_dir_all(data_dir).expect("Failed to create data directory");

    let mut nonce = vec![0u8; 24];
    OsRng.fill_bytes(&mut nonce);
    let nonce = XNonce::from_slice(&nonce);

    let ciphertext = match state.encryptor.encrypt(nonce, data.value.as_bytes()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let file_path = format!("{}/{}.dat", data_dir, data.key);
    let mut file = match OpenOptions::new().write(true).create(true).open(&file_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if file.write_all(&nonce).is_err() || file.write_all(&ciphertext).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().body("Key-value pair stored successfully")
}









//////////////////////////////////////////////////////////////////////









#[derive(Serialize, Deserialize)]
pub struct LoadRequest {
    pub key: String,
}

#[post("/load")]
async fn load(data: web::Json<LoadRequest>, state: web::Data<AppState>) -> impl Responder {
    let data_dir = "data";
    fs::create_dir_all(data_dir).expect("Failed to create data directory");

    let file_path = format!("{}/{}.dat", data_dir, data.key);
    let mut file = match OpenOptions::new().read(true).open(&file_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("File not found"),
    };

    let mut nonce = vec![0u8; 24];
    if let Err(_) = file.read_exact(&mut nonce) {
        return HttpResponse::InternalServerError().finish();
    }

    let mut ciphertext = Vec::new();
    if let Err(_) = file.read_to_end(&mut ciphertext) {
        return HttpResponse::InternalServerError().finish();
    }

    let nonce = XNonce::from_slice(&nonce);
    let plaintext = match state.encryptor.decrypt(nonce, ciphertext.as_ref()) {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match String::from_utf8(plaintext) {
        Ok(text) => HttpResponse::Ok().body(text),
        Err(_) => HttpResponse::InternalServerError().body("Failed to convert plaintext to string"),
    }
}









//////////////////////////////////////////////////////////////////////








#[post("/generate_key")]
async fn generate_key() -> impl Responder {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let key = Key::from_slice(&key_bytes);

    // convert the key to hex so it's easier to print
    let hex_key = hex::encode(key);

    // return the hex key as a response
    HttpResponse::Ok().body(format!("Generated key: {}", hex_key))
}








//////////////////////////////////////////////////////////////////////








// #[post("/login")]
// async fn login() -> impl Responder {
//     let key_bytes = match hex::decode(&hex_key.0) {
//         Ok(bytes) => bytes;
//         Err(_) => return HttpResponse::BadRequest().body("Invalid key format.");
//     }
// }
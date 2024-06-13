use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce, aead::{Aead, KeyInit}};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct StoreRequest {
    key: String,
    value: String,
}

struct AppState {
    encryptor: XChaCha20Poly1305,
}

#[post("/store")]
async fn store(data: web::Json<StoreRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut nonce = vec![0u8; 24];
    OsRng.fill_bytes(&mut nonce);
    let nonce = XNonce::from_slice(&nonce);

    let ciphertext = match state.encryptor.encrypt(nonce, data.value.as_bytes()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let file_path = format!("data/{}.dat", data.key);
    let mut file = match OpenOptions::new().write(true).create(true).open(&file_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if file.write_all(&nonce).is_err() || file.write_all(&ciphertext).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().body("Key-value pair stored successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = Key::from_slice(&[0; 32]); // Use a fixed key for simplicity
    let encryptor = XChaCha20Poly1305::new(&key);

    let state = web::Data::new(AppState { encryptor });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())  // Correct usage of Data's internal Arc for state management
            .service(store)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

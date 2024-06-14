use actix_web::{web, App, HttpServer, HttpResponse, Responder, post, middleware::Logger};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce, aead::{Aead, KeyInit}};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

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

#[derive(Serialize, Deserialize)]
struct LoadRequest {
    key: String,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let key = Key::from_slice(&[0; 32]);
    let encryptor = XChaCha20Poly1305::new(&key);

    let state = web::Data::new(AppState { encryptor });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(store)
            .service(load)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

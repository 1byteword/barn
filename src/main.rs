use actix_web::{web, App, HttpServer, HttpResponse, post, Responder};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce, aead::{Aead, KeyInit}};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct StoreRequest {
    key: String,
    value: String,
}

#[derive(Clone)]
struct AppState {
    encryptor: XChaCha20Poly1305,
    store: Arc<Mutex<HashMap<String, Vec<u8>>>>,
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

    let mut store = state.store.lock().unwrap();
    store.insert(data.key.clone(), ciphertext);

    HttpResponse::Ok().body("Key-value pair stored successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = Key::from_slice(&[0; 32]);
    let encryptor = XChaCha20Poly1305::new(&key);

    let data_store = Arc::new(Mutex::new(HashMap::<String, Vec<u8>>::new()));
    let state = AppState { encryptor, store: data_store };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(store)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

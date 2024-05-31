use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;
use std::collections::HashMap;
use log::info;

#[derive(Deserialize)]
struct StoreRequest {
    data: Vec<String>,
}

#[derive(Deserialize)]
struct LoadRequest {
    data: Vec<String>,
}

pub struct AppState {
    pub tokens: Mutex<HashMap<String, String>>,
}

#[post("/store")]
async fn store(state: web::Data<AppState>, req: web::Json<StoreRequest>) -> impl Responder {
    let mut tokens = state.tokens.lock().unwrap();
    let mut i = 1;
    let mut response = HashMap::new();
    
    for item in &req.data {
        let key = format!("field{}", i);
        tokens.insert(key.clone(), item.clone());
        response.insert(key, item.clone());
        i += 1;
    }
    
    info!("Encrypted data: {:?}", response);
    HttpResponse::Ok().json(response)
}

#[post("/load")]
async fn load(state: web::Data<AppState>, req: web::Json<LoadRequest>) -> impl Responder {
    let tokens = state.tokens.lock().unwrap();
    let mut response = Vec::new();
    let mut i = 1;
    
    for item in &req.data {
        let key = format!("field{}", i);
        if let Some(token) = tokens.get(&key) {
            response.push(token.clone());
        } else {
            response.push(format!("{} (not found)", item));
        }
        i += 1;
    }
    
    info!("Decrypted retrieved data: {:?}", response);
    HttpResponse::Ok().json(response)
}

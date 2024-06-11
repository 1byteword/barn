use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;

pub struct AppState {
    pub tokens: Mutex<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct StoreRequest {
    data: Vec<String>,
}

#[derive(Deserialize)]
pub struct LoadRequest {
    data: Vec<String>,
}

pub async fn store(state: web::Data<AppState>, req: web::Json<StoreRequest>) -> impl Responder {
    let mut tokens = state.tokens.lock().unwrap();
    let mut response = HashMap::new();
    let mut i = 1;

    for item in &req.data {
        let key = format!("field{}", i);
        tokens.insert(key.clone(), item.clone());
        response.insert(key, item.clone());
        i += 1;
    }

    HttpResponse::Ok().json(response)
}

pub async fn load(state: web::Data<AppState>, req: web::Json<LoadRequest>) -> impl Responder {
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

    HttpResponse::Ok().json(response)
}

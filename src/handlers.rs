use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;
use std::collections::HashMap;
use log::info;

#[derive(Deserialize)]
struct TokenizeRequest {
    data: Vec<String>,
}

#[derive(Deserialize)]
struct DetokenizeRequest {
    data: Vec<String>,
}

pub struct AppState {
    pub tokens: Mutex<HashMap<String, String>>,
}

#[post("/tokenize")]
async fn tokenize(state: web::Data<AppState>, req: web::Json<TokenizeRequest>) -> impl Responder {
    let mut tokens = state.tokens.lock().unwrap();
    let mut i = 1;
    let mut response = HashMap::new();
    
    for item in &req.data {
        let key = format!("field{}", i);
        tokens.insert(key.clone(), item.clone());
        response.insert(key, item.clone());
        i += 1;
    }
    
    info!("Tokenized data: {:?}", response);
    HttpResponse::Ok().json(response)
}

#[post("/detokenize")]
async fn detokenize(state: web::Data<AppState>, req: web::Json<DetokenizeRequest>) -> impl Responder {
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
    
    info!("Detokenized data: {:?}", response);
    HttpResponse::Ok().json(response)
}

use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Mutex;
use std::collections::HashMap;
use crate::models::{RequestData, TokenizedResponse, DetokenizedResponse};
use log::{info, error};

pub struct AppState {
    pub tokens: Mutex<HashMap<String, String>>,
}

#[post("/tokenize")]
async fn tokenize(data: web::Json<RequestData>, state: web::Data<AppState>) -> impl Responder {
    info!("Received tokenize request: {:?}", data);

    let mut tokens = match state.tokens.lock() {
        Ok(t) => t,
        Err(err) => {
            error!("Failed to acquire lock: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    
    for (i, item) in data.data.iter().enumerate() {
        tokens.insert(format!("field{}", i + 1), item.clone());
    }

    let response = TokenizedResponse {
        tokens: tokens.clone(),
    };

    info!("Tokenize response: {:?}", response);
    HttpResponse::Ok().json(response)
}

#[post("/detokenize")]
async fn detokenize(data: web::Json<RequestData>, state: web::Data<AppState>) -> impl Responder {
    info!("Received detokenize request: {:?}", data);

    let tokens = match state.tokens.lock() {
        Ok(t) => t,
        Err(err) => {
            error!("Failed to acquire lock: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let mut response = Vec::new();

    for (i, item) in data.data.iter().enumerate() {
        let key = format!("field{}", i + 1);
        if let Some(value) = tokens.get(&key) {
            response.push(value.clone());
        } else {
            response.push(format!("{} (not found)", item));
        }
    }

    let response = DetokenizedResponse { response };

    info!("Detokenize response: {:?}", response);
    HttpResponse::Ok().json(response)
}

mod handlers;
mod models;

use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
use std::collections::HashMap;
use handlers::{tokenize, detokenize, AppState};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server");

    let app_state = web::Data::new(AppState {
        tokens: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(tokenize)
            .service(detokenize)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

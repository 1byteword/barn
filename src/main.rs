mod endpoints;

use actix_web::{web, App, HttpServer, middleware::Logger};
use chacha20poly1305::{XChaCha20Poly1305, Key, KeyInit};
use rand::rngs::OsRng;
use rand::RngCore;

struct AppState {
    encryptor: XChaCha20Poly1305,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let key = Key::from_slice(&key_bytes);

    let encryptor = XChaCha20Poly1305::new(&key);

    let state = web::Data::new(AppState { encryptor });

    let logo = r#"
===========================================================
      ________  ________  ________  ________      
     |\   __  \|\   __  \|\   __  \|\   ___  \    
     \ \  \|\ /\ \  \|\  \ \  \|\  \ \  \\ \  \   
      \ \   __  \ \   __  \ \   _  _\ \  \\ \  \  
       \ \  \|\  \ \  \ \  \ \  \\  \\ \  \\ \  \ 
        \ \_______\ \__\ \__\ \__\\ _\\ \__\\ \__\
         \|_______|\|__|\|__|\|__|\|__|\|__| \|__|
         
===========================================================
    "#;

    println!("{}", logo);
    println!("Welcome to the Barnyard Key-Value Store.");
    println!("Starting Barn API server on http://127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(endpoints::store)
            .service(endpoints::load)
            .service(endpoints::generate_key)
            //.service(endpoints::login)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

mod handlers;
mod models;
mod storage;
mod encryption;
mod access_control;
mod kv_silo;

use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use handlers::{store, load, AppState};
use log::info;
use clap::{Parser, Subcommand};
use storage::{ensure_dir_exists};
use encryption::{generate_key, encrypt, decrypt};
use access_control::AccessControl;
use uuid::Uuid;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use shamirsecretsharing::*;

const USER_ID_FILE: &str = "user_id.txt";
const KEY_FILE: &str = "encryption_key.bin";
const DATA_SIZE: usize = 64;

static DEK_SHARES: Lazy<Mutex<HashMap<String, Vec<Vec<u8>>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "API Server to encrypt, store, and retrieve data.")]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

struct User {
    username: String,
    password_hash: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    Serve {
        #[clap(short, long, default_value = "127.0.0.1:8000")]
        address: String,
    },

    Store {
        #[clap(short, long)]
        data: Vec<String>,
    },

    Load {
        #[clap(short, long)]
        data: Vec<String>,
    },
}

fn get_or_create_user_id() -> Uuid {
    if let Ok(contents) = fs::read_to_string(USER_ID_FILE) {
        if let Ok(uuid) = Uuid::parse_str(contents.trim()) {
            return uuid;
        }
    }
    let user_id = Uuid::new_v4();
    fs::write(USER_ID_FILE, user_id.to_string()).expect("Unable to write user_id file");
    user_id
}

fn get_or_create_key() -> Vec<u8> {
    if let Ok(mut file) = File::open(KEY_FILE) {
        let mut key = vec![0; 32];  // Ensure the key is 32 bytes
        file.read_exact(&mut key).expect("Unable to read key file");
        return key;
    }
    let key = generate_key();
    let mut file = File::create(KEY_FILE).expect("Unable to create key file");
    file.write_all(&key).expect("Unable to write key file");
    key
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting your Barnyard...");

    let args = Args::parse();
    let base_dir = "secure_data";
    ensure_dir_exists(&base_dir).unwrap();

    let user_id = get_or_create_user_id();
    let key = get_or_create_key();

    let mut access_control = AccessControl::new();
    access_control.grant_access(user_id, format!("{}/my_secret_document.txt", base_dir));

    let path = format!("{}/my_secret_document.txt", base_dir);

    match args.command {
        Command::Serve { address } => {
            let app_data = web::Data::new(AppState {
                tokens: Mutex::new(std::collections::HashMap::new()),
            });

            HttpServer::new(move || {
                App::new()
                    .app_data(app_data.clone())
                    .service(web::resource("/store").route(web::post().to(store)))
                    .service(web::resource("/load").route(web::post().to(load)))
            })
            .bind(address)?
            .run()
            .await
        }

        Command::Store { data } => {
            let data_str = data.join(" ");
            let mut data_bytes = data_str.as_bytes().to_vec();
            data_bytes.resize(DATA_SIZE, 0);

            let shares = create_shares(&data_bytes, 5, 3).unwrap();
            
            println!("Farmer, store these bales in a safe place:");
            for (i, share) in shares.iter().enumerate() {
                print!("Bale {}: ", i + 1);
                for byte in share {
                    print!("{} ", byte);
                }
                println!();
            }

            let (nonce, encrypted_value) = encrypt(&data_bytes, &key);

            let kv_store = kv_silo::KVStore::new();
            kv_store.set_secret("my_secret".to_string(), nonce.clone(), encrypted_value.clone()).await?;

            kv_store.save_to_file_encrypted(&path, &key).await?;
            //access_control.grant_access(user_id, path.clone());

            info!("Tokenized data and saved to {}", path);
            println!("Your data has been tokenized and saved to {}", path);
            Ok(())
        }

        Command::Load { data: _ } => {
            let mut input_shares = Vec::new();

            for i in 0..3 {
                println!("Enter Bale {}: ", i + 1);
                let mut share_input = String::new();
                std::io::stdin().read_line(&mut share_input).unwrap();
                let share: Vec<u8> = share_input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
                input_shares.push(share);
            }

            let recovered_dek = combine_shares(&input_shares).unwrap().unwrap();

            let kv_store = kv_silo::KVStore::new();
            kv_store.load_from_file_encrypted(&path, &recovered_dek).await?;

            if let Some(secret) = kv_store.get_secret("my_secret").await {
                let decrypted_value = decrypt(&secret.iv, &recovered_dek).expect("Failed to decrypt");
                let decrypted_str = String::from_utf8(decrypted_value).unwrap();
                println!("Decrypted value: {}", decrypted_str);
            } else {
                println!("Secret not found");
            }

            Ok(())
        }
    }
}

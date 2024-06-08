mod handlers;
mod models;
mod storage;
mod encryption;
mod access_control;
mod kv_silo;
mod shamir;

use actix_web::{App, HttpServer, HttpResponse, Responder, web};
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
use rand::rngs::OsRng;
use rand::RngCore;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use kv_silo::KVStore;
use shamir::{make_random_shares, reconstruct_secret, PRIME};

const USER_ID_FILE: &str = "user_id.txt";
const KEY_FILE: &str = "encryption_key.bin";

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

// fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
//     hash(password, DEFAULT_COST)
// }

// fn register_user(username: String, password: String) -> Result<(), String> {
//     println!("Registering user...");
//     let password_hash = hash_password(&password).map_err(|e| e.to_string())?;
//     println!("User {} registered successfully.", username);
//     Ok(())
// }

// fn authenticate_user(username: &str, password: &str) -> Result<bool, String> {
//     let user = User {
//         username: username.to_string(),
//         password_hash: hash_password(password).unwrap(),
//     };
//     match verify(password, &user.password_hash) {
//         Ok(matching) => Ok(matching),
//         Err(e) => Err(e.to_string()),
//     }
// }

// async fn login(info: web::Json<User>) -> impl Responder {
//     if authenticate_user(&info.username, &info.password_hash).unwrap_or(false) {
//         HttpResponse::Ok().body("Login successful")
//     } else {
//         HttpResponse::BadRequest().body("Login failed. Invalid username or password.")
//     }
// }


fn store_share(id: String, share: Vec<u8>) {
    let mut shares_map = DEK_SHARES.lock().unwrap();
    shares_map.entry(id).or_insert_with(Vec::new).push(share);
}

fn retrieve_shares(id: String) -> Option<Vec<Vec<u8>>> {
    let shares_map = DEK_SHARES.lock().unwrap();
    shares_map.get(&id).cloned()
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
        let mut key = vec![0; 32];
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
            let dek = {
                let mut key = [0u8; 32];
                OsRng.fill_bytes(&mut key);
                BigInt::from_bytes_le(num_bigint::Sign::Plus, &key).to_i64().unwrap()
            };

            let prime = BigInt::from_str(PRIME).unwrap();

            let shares = make_random_shares(dek, 3, 6, &prime);
            
            println!("Farmer, store these bales in a safe place:");
            for (i, share) in shares.iter().enumerate() {
                println!("Bale {}: {:?}", i + 1, share);
            }

            let (iv, encrypted_value) = encrypt(&dek, data_str.as_bytes());

            let kv_store = KVStore::new();
            kv_store.set_secret("my_secret".to_string(), iv.clone(), encrypted_value.clone()).await?;


            kv_store.save_to_file_encrypted(&path, &key).await?;
            //access_control.grant_access(user_id, path.clone());

            info!("Tokenized data and saved to {}", path);
            println!("Your data has been tokenized and saved to {}", path);
            Ok(())
        }

        Command::Load { data: _ } => {
            let mut input_shares = Vec::new();

            for i in 0..3 {
                println!("Enter Bale {}: (format x y)", i + 1);
                let mut share_input = String::new();
                std::io::stdin().read_line(&mut share_input).unwrap();
                let parts: Vec<&str> = share_input.trim().split_whitespace().collect();

                if parts.len() == 2 {
                    let x = BigInt::from_str(parts[0]).unwrap();
                    let y = BigInt::from_str(parts[1]).unwrap();
                    input_shares.push((x, y));
                }
            }

            let prime = BigInt::from_str(PRIME).unwrap();
            let recovered_dek = reconstruct_secret(&input_shares, &prime);

            let kv_store = KVStore::new();
            kv_store.load_from_file_encrypted(&path, &key).await?;

            if let Some((iv, encrypted_value)) = kv_store.get_secret("my_secret").await {
                let decrypted_value = decrypt(&recovered_dek.to_bytes_le().1, &iv, &encrypted_value);
                let decrypted_str = String::from_utf8(decrypted_value).unwrap();
                println!("Decrypted value: {}", decrypted_str);
            } else {
                println!("Secret not found");
            }

            Ok(())
        }
    }
}

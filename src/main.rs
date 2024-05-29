mod handlers;
mod models;

use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
use std::collections::HashMap;
use handlers::{tokenize, detokenize, AppState};
use log::info;
use clap::Parser;
use clap::Subcommand;
use reqwest::Client;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about="This is a simple API server that tokenizes and detokenizes data.")]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1:8000")]
    address: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Serve {
        // bind server
        #[clap(short, long, default_value = "127.0.0.1:8000")]
        address: String,
    },

    // data tokenizer
    Tokenize {
        #[clap(short, long)]
        data: Vec<String>,
    },

    // data detokenizer
    Detokenize {
        #[clap(short, long)]
        data: Vec<String>,
    },
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server");

    let args = Args::parse();

    match args.command {
        Command::Serve { address } => {
            let app_state = web::Data::new(AppState {
                tokens: Mutex::new(HashMap::new()),
            });

            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .service(tokenize)
                    .service(detokenize)    
            })
            .bind(&address)?
            .run()
            .await
        }

        Command::Tokenize { data } => {
            let client = Client::new();
            let response = client.post("http://127.0.0.1:8000/tokenize")
                .json(&models::RequestData { data })
                .send()
                .await
                .expect("Failed to send request");
            let body = response.text().await.expect("Failed to read response text.");
            println!("{}", body);
            Ok(())
        }

        Command::Detokenize { data } => {
            let client = Client::new();
            let response = client.post("http://127.0.0.1:8000/detokenize")
                .json(&models::RequestData { data })
                .send()
                .await
                .expect("Failed to send request");
            let body = response.text().await.expect("Failed to read response text.");
            println!("{}", body);
            Ok(())
        }
    }
}
use std::env;

use client::Client;
use common::util::default_error_handler;
use config::ClientConfig;

mod client;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let client_config = ClientConfig::from_args(&args).unwrap_or_default();
    let mut client = Client::new(client_config)
        .await
        .unwrap_or_else(|e| default_error_handler(e));

    client
        .run()
        .await
        .unwrap_or_else(|e| default_error_handler(e));
}

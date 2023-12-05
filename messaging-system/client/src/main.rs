use std::env;

use client::Client;
use common::util;
use config::ClientConfig;

mod client;
mod config;
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let client_config = ClientConfig::from_args(&args).unwrap_or_default();
    let mut client = Client::new(client_config).unwrap_or_else(|e| util::default_error_handler(e));
    client
        .run()
        .unwrap_or_else(|e| util::default_error_handler(e));
}

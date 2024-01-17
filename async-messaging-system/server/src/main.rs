use std::env;

use common::util::default_error_handler;
use config::ServerConfig;
use server::Server;

mod config;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_default();
    let mut server = Server::new(server_config)
        .await
        .unwrap_or_else(|e| default_error_handler(e));

    server
        .run()
        .await
        .unwrap_or_else(|e| default_error_handler(e));
}

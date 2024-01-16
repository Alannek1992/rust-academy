use std::{env, process};

use common::util;
use config::ServerConfig;
use server::Server;

mod config;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_default();
    let server = Server::new(server_config).await.unwrap_or_else(|e| {
        util::print_msg_to_stderr(e);
        process::exit(1)
    });

    server.run().await.unwrap_or_else(|e| {
        util::print_msg_to_stderr(e);
        process::exit(1)
    });
}

use std::env;

use common::{config::ServerConfig, util};
use server::Server;

mod server;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_default();
    let mut server = Server::new(server_config).unwrap_or_else(|e| util::default_error_handler(e));

    server
        .run()
        .unwrap_or_else(|e| util::default_error_handler(e));
}

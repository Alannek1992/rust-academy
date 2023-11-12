use std::{env, net::TcpListener};

use common::util::{self, ColorFacade};
use config::ServerConfig;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_else(|e| {
        let default_config = ServerConfig::default();
        util::print_error_to_stdout(e);
        default_config
    });

    let listener = TcpListener::bind(server_config.to_string()).expect(&format!(
        "Failed to bind TcpListener to address: {}",
        server_config.to_string()
    ));

    util::print_msg_to_stdout(
        &format!("Listening on: {}", server_config.to_string()),
        ColorFacade::Yellow,
    );
}

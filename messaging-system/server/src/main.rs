use std::{env, error::Error as StdError, process};

use common::{
    config::ServerConfig,
    util::{self, ColorFacade},
};
use server::Server;

mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_default();
    let socket_address = server_config
        .to_socket_address()
        .unwrap_or_else(|e| handle_error(e));
    let mut server = Server::new(socket_address).unwrap_or_else(|e| handle_error(e));

    util::print_msg_to_stdout(
        &format!("Listening on: {}", server_config),
        ColorFacade::Yellow,
    );

    server.run().unwrap_or_else(|e| handle_error(e));
}

fn handle_error(e: Box<dyn StdError>) -> ! {
    util::print_error_to_stdout(e);
    process::exit(1);
}

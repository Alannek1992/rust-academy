use std::{env, error::Error as StdError, process};

use client::Client;
use common::util::{self, ColorFacade};
use config::ClientConfig;

mod client;
mod config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let client_config = ClientConfig::from_args(&args).unwrap_or_default();
    let socket_address = client_config
        .to_socket_address()
        .unwrap_or_else(|e| handle_error(e));

    let mut client = Client::new(socket_address).unwrap_or_else(|e| handle_error(e));

    util::print_msg_to_stdout(
        &format!("Connected to: {}", client_config),
        ColorFacade::Yellow,
    );

    client.run().unwrap_or_else(|e| handle_error(e));
}

fn handle_error(e: Box<dyn StdError>) -> ! {
    util::print_error_to_stdout(e);
    process::exit(1);
}

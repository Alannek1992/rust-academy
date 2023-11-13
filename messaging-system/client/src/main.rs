use std::{env, net::TcpStream, process};

use common::{
    config::ServerConfig,
    error::Error,
    util::{self, ColorFacade},
};
use stdio_processor::StdioProcessor;

mod stdio_processor;
fn main() {
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_else(|e| {
        let default_config = ServerConfig::default();
        util::print_error_to_stdout(e);
        default_config
    });

    let stream = TcpStream::connect(server_config.to_string()).unwrap_or_else(|e| {
        util::print_error_to_stdout(Error::new(&format!(
            "Cannot connect to the server: {}. Details: {}",
            server_config.to_string(),
            e.to_string()
        )));
        process::exit(1);
    });

    util::print_msg_to_stdout(
        &format!("Connected to: {}", server_config.to_string()),
        ColorFacade::Yellow,
    );

    let mut stdio_processor = StdioProcessor::new(stream);
    stdio_processor.run();
}

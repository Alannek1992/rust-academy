use std::{
    collections::HashMap,
    env,
    net::{TcpListener, TcpStream},
    process,
};

use common::{
    api::{MessageEnvelope, Username},
    config::ServerConfig,
    error::Result,
    util::{self, ColorFacade},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_config = ServerConfig::from_args(&args).unwrap_or_else(|e| {
        let default_config = ServerConfig::default();
        util::print_error_to_stdout(e);
        default_config
    });

    let listener = TcpListener::bind(server_config.to_string()).unwrap_or_else(|e| {
        util::print_error_to_stdout(Box::new(e));
        process::exit(1);
    });

    util::print_msg_to_stdout(
        &format!("Listening on: {}", server_config.to_string()),
        ColorFacade::Yellow,
    );

    let mut connections: HashMap<Username, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &mut connections) {
                    util::print_error_to_stdout(e);
                }
            }
            Err(e) => {
                util::print_error_to_stdout(Box::new(e));
            }
        }
    }
}

fn handle_client(
    mut stream: TcpStream,
    existing_connections: &mut HashMap<Username, TcpStream>,
) -> Result<()> {
    let msg = MessageEnvelope::read_frame(&mut stream)?;
    let msg = MessageEnvelope::deserialize(&msg)?;

    util::print_msg_to_stdout(
        &format!("Connection established with user: {}", msg.from_user),
        ColorFacade::Green,
    );

    Ok(())
}

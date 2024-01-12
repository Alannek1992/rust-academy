use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;

use anyhow::{anyhow, Result};
use common::api::{Message, MessageEnvelope};
use common::config::ServerConfig;
use common::error::MsgSystemError;
use common::util::{self, ColorFacade};
use log::{error, trace};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const SERVER_TOKEN: Token = Token(0);

pub struct Server {
    listener: TcpListener,
    clients: HashMap<Token, TcpStream>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Result<Self> {
        let address = config.to_socket_address()?;
        let listener = TcpListener::bind(address)?;
        util::print_msg_to_stdout(&format!("Listening on: {}", config), ColorFacade::Yellow);

        Ok(Self {
            listener,
            clients: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut poll = Poll::new()?;
        poll.registry()
            .register(&mut self.listener, SERVER_TOKEN, Interest::READABLE)?;
        self.start(&mut poll)?;

        Ok(())
    }

    fn start(&mut self, poll: &mut Poll) -> Result<()> {
        let mut events = Events::with_capacity(1024);
        trace!("Starting polling events");

        loop {
            poll.poll(&mut events, Some(Duration::from_millis(100)))?;

            for event in events.iter() {
                let result = match event.token() {
                    SERVER_TOKEN => {
                        // accept new clients
                        self.accept_client(poll)
                    }
                    token => {
                        // handle events for existing clients
                        self.handle_client_event(token, event)
                    }
                };

                if let Err(e) = result {
                    error!("Error occured when polling event: {}", e);
                }
            }
        }
    }

    fn accept_client(&mut self, poll: &mut Poll) -> Result<()> {
        let (mut stream, _) = self.listener.accept()?;
        let token = Token(self.clients.len() + 1);
        trace!("New client connected. Registering as: {:?}", token);
        poll.registry()
            .register(&mut stream, token, Interest::READABLE)?;
        self.clients.insert(token, stream);
        Ok(())
    }

    fn handle_client_event(&mut self, token: Token, event: &Event) -> Result<()> {
        if !event.is_readable() {
            return Err(anyhow!("The event is not readable"));
        }

        let mut stream = self.clients.get_mut(&token).ok_or(anyhow!(
            "The TCP stream for following token: {} not found",
            token.0
        ))?;

        // at this point the TCP stream between client and server is established


        let msg_frame = MessageEnvelope::read_frame(&mut stream)?;
        let msg_envelope = MessageEnvelope::deserialize(&msg_frame)?;
        trace!(
            "Existing client: {} sent a message: {:?}",
            msg_envelope.from_user,
            msg_envelope.content
        );

        if msg_envelope.content == Message::Exit {
            trace!("Client was disconnected from the server");
            self.clients.remove(&token).unwrap();
        }

        let msg = msg_envelope.serialize()?;

        // writing to all clients except the sender
        self.clients
            .iter_mut()
            .filter(|(t, _)| **t != token)
            .for_each(|(_, s)| {
                s.write_all(&msg).unwrap_or_else(|e| {
                    util::print_error_to_stdout(Error::new(&format!(
                        "Cannot write into TCP stream: {}",
                        e
                    )));
                });
            });

        Ok(())
    }
    fn send_error_response(stream: &TcpStream, error: MsgSystemError) {
        let msg_env = MessageEnvelope::new(None, Some(error));
    }
}

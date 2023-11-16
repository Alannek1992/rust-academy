use std::collections::HashMap;
use std::io::Write;
use std::net::SocketAddr;
use std::time::Duration;

use common::api::MessageEnvelope;
use common::error::{Error, Result};
use common::util;
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const SERVER_TOKEN: Token = Token(0);

pub struct Server {
    listener: TcpListener,
    clients: HashMap<Token, TcpStream>,
}

impl Server {
    pub fn new(address: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(address)?;
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
                    util::print_error_to_stdout(e);
                }
            }
        }
    }

    fn accept_client(&mut self, poll: &mut Poll) -> Result<()> {
        let (mut stream, _) = self.listener.accept()?;
        let token = Token(self.clients.len() + 1);
        poll.registry()
            .register(&mut stream, token, Interest::READABLE)?;
        self.clients.insert(token, stream);
        Ok(())
    }

    fn handle_client_event(&mut self, token: Token, event: &Event) -> Result<()> {
        if !event.is_readable() {
            return Err(Error::new("The event is not readable"));
        }
        let mut stream = self.clients.get_mut(&token).ok_or(Error::new(&format!(
            "The TCP stream for following token: {} not found",
            token.0
        )))?;
        let msg = MessageEnvelope::read_frame(&mut stream)?;

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
}

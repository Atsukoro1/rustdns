use std::net::SocketAddr;
use slog::warn;
use crate::{
    parser::dns::DNS, 
    LOGGER
};

pub struct Handler {
    pub datagram: Option<DNS>,
    pub sent_from: Option<SocketAddr>
}

pub trait HandlerT {
    fn new() -> Handler;
    fn handle(&mut self, buf: &[u8], from: SocketAddr);
}

impl HandlerT for Handler {
    fn new() -> Handler {
        Handler { datagram: None, sent_from: None }
    }

    fn handle(&mut self, buf: &[u8], from: SocketAddr) {
        self.sent_from = Some(from);

        match DNS::from(&*buf) {
            Ok(result) => {
                self.datagram = Some(result)
            },

            Err(..) => {
                warn!(
                    LOGGER, 
                    "An invalid datagram was sent";
                    "Sent from: " => from.to_string()
                );
            }
        };

        println!("{:?}", self.datagram);
    }
}
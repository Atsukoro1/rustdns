use std::net::SocketAddr;
use slog::warn;
use crate::{
    parser::{dns::DNS, rcode::ResponseCode, r#type::Type, opcode::OpCode}, 
    LOGGER, SOCKET
};

use super::question::{QuestionHandler, QuestionHandlerT};

pub struct Handler {
    pub datagram: DNS,
    pub sent_from: Option<SocketAddr>
}

pub trait HandlerT {
    fn new() -> Handler;
    fn handle(&mut self, buf: &[u8], from: SocketAddr);
    fn resolve_questions(&mut self);
    fn send_fail_response(&mut self, code: ResponseCode);
}

impl HandlerT for Handler {
    fn new() -> Handler {
        Handler { 
            datagram: DNS::new(), 
            sent_from: None
        }
    }

    fn send_fail_response(&mut self, code: ResponseCode) {
        let mut response_datagram = DNS::new();

        // Set header
        response_datagram.header.qr = Type::Response;
        response_datagram.header.error_code = code;
        response_datagram.header.op_code = self.datagram.header.op_code;
        response_datagram.header.truncated = false;
        response_datagram.header.id = self.datagram.header.id;

        SOCKET.send_to::<SocketAddr>(
            &*response_datagram.bytes().unwrap(), 
            self.sent_from.unwrap()
        )
        .expect("Failed to send!");
    }

    fn resolve_questions(&mut self) {
        for i in 0..self.datagram.questions.len() {
            match QuestionHandler::new()
                .handle(self.datagram.questions[i].clone()) {
                    Ok(result_rf) => {
                        self.datagram.answer.as_mut()
                            .unwrap()
                            .push(result_rf);
                    },

                    Err(..) => {
                        self.send_fail_response(ResponseCode::FormatError);
                        break;
                    }
                }
        }
    }

    fn handle(&mut self, buf: &[u8], from: SocketAddr) {
        self.sent_from = Some(from);

        match DNS::from(&*buf) {
            Ok(result) => {
                self.datagram = result;
                println!("{:?}", self.datagram);
                self.resolve_questions();
            },

            Err(..) => {
                warn!(
                    LOGGER, 
                    "An invalid datagram was sent";
                    "Sent from: " => from.to_string()
                );
            }
        };
    }
}
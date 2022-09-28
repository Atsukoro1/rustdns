use std::net::SocketAddr;
use slog::warn;
use crate::{
    parser::{
        dns::DNS, 
        rcode::ResponseCode, 
        r#type::Type
    }, 
    LOGGER, SOCKET
};
use super::{question::{
    QuestionHandler, 
    QuestionHandlerT
}, transport::TransportProto};

/// This struct takes an ownership of the datagram and will process it.
pub struct Handler {
    pub datagram: DNS,
    pub sent_from: Option<SocketAddr>
}

#[async_trait::async_trait]
pub trait HandlerT {
    /// Creates a new datagram handler
    fn new() -> Handler;

    /// Will parse the datagram and send a response back if parsing fails,
    /// once done, datagram is moved to resolve_questions function for further
    /// processing
    async fn handle(&mut self, buf: &[u8], from: SocketAddr);

    /// Will send each question to question handler that will check validity of
    /// each one and then resolve and return back result.
    /// Can send "fail response" if processing fails
    async fn resolve_questions(&mut self);

    /// Helper function for sending responses when resolving fails
    fn send_fail_response(&mut self, code: ResponseCode);
}

#[async_trait::async_trait]
impl HandlerT for Handler {
    fn new() -> Handler {
        Handler { 
            datagram: DNS::new(), 
            sent_from: None
        }
    }

    fn send_fail_response(&mut self, code: ResponseCode) {
        let mut response_datagram = DNS::new();

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

    async fn resolve_questions(&mut self) {
        for i in 0..self.datagram.questions.as_ref().unwrap().len() {
            match QuestionHandler::new()
                .handle(self.datagram.questions.as_ref().unwrap()[i].clone()).await {
                    Ok(result_rf) => {
                        self.datagram.answer.as_mut()
                            .unwrap()
                            .push(result_rf);
                    },

                    Err(code) => {
                        self.send_fail_response(code);
                        break;
                    }
                }
        }
    }

    async fn handle(&mut self, buf: &[u8], from: SocketAddr) {
        self.sent_from = Some(from);

        match DNS::from(&*buf, TransportProto::UDP) {
            Ok(result) => {
                self.datagram = result;
                self.resolve_questions().await;
            },

            Err(..) => {
                warn!(
                    LOGGER, 
                    "An invalid datagram was sent";
                    "Sent from: " => from.to_string()
                );

                self.send_fail_response(ResponseCode::FormatError);
            }
        };
    }
}
use crate::parser::{question::DNSQuestion, rcode::ResponseCode, resource::DNSResourceFormat};

pub struct QuestionHandler {
    question: Option<DNSQuestion>
}

pub trait QuestionHandlerT {
    fn new() -> QuestionHandler;

    fn handle(
        &mut self, inp: DNSQuestion
    ) -> Result<DNSResourceFormat, ResponseCode>;
}

impl QuestionHandlerT for QuestionHandler {
    fn new() -> QuestionHandler {
        QuestionHandler { question: None }
    }

    fn handle(&mut self, inp: DNSQuestion) -> Result<DNSResourceFormat, ResponseCode> {
        self.question = Some(inp);
        Err(ResponseCode::FormatError)
    }
}
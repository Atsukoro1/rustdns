use redis::Commands;

use crate::{parser::{
    question::DNSQuestion, 
    rcode::ResponseCode, 
    resource::DNSResourceFormat
}, CACHEMANAGER};

pub struct QuestionHandler {
    question: Option<DNSQuestion>
}

#[async_trait::async_trait]
pub trait QuestionHandlerT {
    fn new() -> QuestionHandler;

    async fn handle(
        &mut self, inp: DNSQuestion
    ) -> Result<DNSResourceFormat, ResponseCode>;

    async fn check_if_exists(name: &String) -> bool;
}

#[async_trait::async_trait]
impl QuestionHandlerT for QuestionHandler {
    fn new() -> QuestionHandler {
        QuestionHandler { question: None }
    }

    async fn check_if_exists(name: &String) -> bool {
        let mut cm = CACHEMANAGER.lock().await;
        let r_inst = cm
            .redis_instance
            .as_mut()
            .unwrap()
            .get_mut();

        match r_inst.get::<String, String>(format!("TLD:{}", name)) {
            Ok(..) => return true,
            Err(..) => return false
        };
    }

    async fn handle(&mut self, inp: DNSQuestion) -> Result<DNSResourceFormat, ResponseCode> {
        self.question = Some(inp);
        
        let exists: bool = Self::check_if_exists(
            &self.question.as_ref()
                .unwrap()
                .name
                .split(".")
                .last()
                .unwrap()
                .to_string()
        ).await;

        if !exists {
            return Err::<DNSResourceFormat, ResponseCode>(
                ResponseCode::NameError
            );
        }

        Err(ResponseCode::FormatError)
    }
}
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
    /// Create a new instance of question handler
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
                .to_uppercase()
        ).await;

        if !exists {
            return Err(
                ResponseCode::NameError
            );
        }

        /*
            This code is here only for now because resource record 
            format is not implemented
        */
        Err(ResponseCode::ServerFailure)
    }
}
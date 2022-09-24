use std::{net::SocketAddr, str::FromStr};
use redis::Commands;
use fancy_regex::Regex;
use crate::{parser::{
    question::DNSQuestion, 
    rcode::ResponseCode, 
    resource::DNSResourceFormat
}, CACHEMANAGER, 
    cache::modules::rootserver::RootServer
};

pub struct QuestionHandler {
    /// Holding the question by the end user
    question: Option<DNSQuestion>,

    /// Root server assigned for this particular question
    root_server: Option<RootServer>,

    /// TLD nameserver assigned for this particular question
    tld_ns: Option<SocketAddr>,

    /// Authoritative nameserver assigned for this particular question
    authoritative_ns: Option<SocketAddr>
}

#[async_trait::async_trait]
pub trait QuestionHandlerT {
    /// Create a new instance of question handler
    fn new() -> QuestionHandler;

    /// Handle new domain name
    async fn handle(
        &mut self, inp: DNSQuestion
    ) -> Result<DNSResourceFormat, ResponseCode>;

    /// Check if TLD exists in IANA database
    async fn check_if_exists(name: &String) -> bool;

    /// Check is string is fully qualified domain name
    /// 
    /// This function does not check for TLD validity, only for the 
    /// compelete fqdn pattern
    fn check_fqdn_validity(fqdn: &String) -> bool;
}

#[async_trait::async_trait]
impl QuestionHandlerT for QuestionHandler {
    fn new() -> QuestionHandler {
        QuestionHandler { 
            question: None,
            root_server: None,
            tld_ns: None,
            authoritative_ns: None
        }
    }

    fn check_fqdn_validity(fqdn: &String) -> bool {
        Regex::from_str(r"(?=^.{4,253}$)(^((?!-)[a-zA-Z0-9-]{1,63}(?<!-)\.)+[a-zA-Z]{2,63}$)")
            .unwrap()
            .is_match(fqdn.as_str())
            .unwrap()
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

        let valid = Self::check_fqdn_validity(
            &self.question
                .as_ref()
                .unwrap()
                .name
        );
        println!("{}", valid);

        if !valid {
            return Err(
                ResponseCode::NameError
            )
        }
        
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
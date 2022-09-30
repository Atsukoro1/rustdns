use std::{net::{SocketAddr}, str::FromStr};
use redis::Commands;
use fancy_regex::Regex;
use crate::{parser::{
    question::DNSQuestion, 
    rcode::ResponseCode, 
    resource::DNSResourceFormat, dns::DNS, r#type::Type, opcode::OpCode, qtype::QuestionType, qclass::QuestionClass
}, CACHEMANAGER, 
    cache::modules::rootserver::RootServer,
};
use super::transport;

pub struct QuestionHandler {
    /// Holding the question by the end user
    question: Option<DNSQuestion>,

    /// Root server assigned for this particular question
    root_server: Option<RootServer>,

    /// TLD nameserver assigned for this particular question
    tld_ns: Option<SocketAddr>,

    /// Authoritative nameserver assigned for this particular question
    authoritative_ns: Option<Vec<SocketAddr>>
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

    /// Will get available root server and 
    async fn query_rootserver(&mut self) -> Result<(), ResponseCode>;
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

        // Rewrite this in FQDN struct later
        
        let valid = Self::check_fqdn_validity(
            &self.question.as_ref() 
                .unwrap()
                .name
                .to_string()
        );

        if !valid {
            return Err(
                ResponseCode::NameError
            )
        }
        
        let exists: bool = Self::check_if_exists(
            &&self.question.as_ref()
                .unwrap()
                .name
                .tld
        ).await;

        if !exists {
            return Err(
                ResponseCode::NameError
            );
        }

        match Self::query_rootserver(&mut self).await {
            Ok(..) => {

            },

            Err(..) => {
                return Err(
                    ResponseCode::ServerFailure
                );
            }
        }

        /*
            This code is here only for now because resource record 
            format is not implemented
        */
        Err(ResponseCode::ServerFailure)
    }

    async fn query_rootserver(&mut self) -> Result<(), ResponseCode> {
        let mut cm = CACHEMANAGER.lock().await;
        let r_inst = cm
            .redis_instance
            .as_mut()
            .unwrap()
            .get_mut();

        let r_inst = r_inst.get::<&str, String>("ROOTS:A")
            .unwrap();
        /*
            Cache manager will be dropped at the end of the file which is
            after the request and that's extra latency. Instead it will be 
            freed here
        */
        std::mem::drop(cm);

        let question = self.question
            .as_ref()
            .unwrap();

        // Building question for root server
        let mut root_s_datagram = DNS::new();
        root_s_datagram.header.qr = Type::Query;
        root_s_datagram.header.truncated = false;
        root_s_datagram.header.recursion_desired = true;
        root_s_datagram.header.question_count = 1;
        root_s_datagram.header.op_code = OpCode::Query;
        root_s_datagram.header.id = 10039;
        root_s_datagram.questions = Some(vec![DNSQuestion { 
            name: question.name.clone(), 
            qtype: QuestionType::NS, 
            class: QuestionClass::IN 
        }]);

        let hostname = r_inst.split(" ")
            .into_iter()
            .next()
            .unwrap()
            .split("_")
            .nth(2)
            .unwrap();
        
        /*
            Create transport that will handle the whole TCP/UDP mess situation
        */
        let pkt: DNS = match transport::onetime_transport(
            &root_s_datagram.bytes().unwrap(), 
            format!("{}:{}", hostname, 53).parse::<SocketAddr>()
                .unwrap(),
            None
        ).await {
            Ok(packet) => packet,
            Err(..) => {
                return Result::Err(
                    ResponseCode::ServerFailure
                );
            }
        };

        println!("{:?}", pkt);

        todo!();
    }
}
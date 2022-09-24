use std::{net::{SocketAddr, UdpSocket, IpAddr, Ipv4Addr}, str::FromStr};
use redis::Commands;
use fancy_regex::Regex;
use crate::{parser::{
    question::DNSQuestion, 
    rcode::ResponseCode, 
    resource::DNSResourceFormat, dns::DNS, r#type::Type, opcode::OpCode
}, CACHEMANAGER, 
    cache::modules::rootserver::RootServer, CONFIG
};

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
    async fn query_rootserver(&mut self, socket: &mut UdpSocket) -> Result<(), ResponseCode>;
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
            This socket will be used to fetch all nameservers and stuff like that.

            We're setting the port number to zero because the OS will automatically
            allocate some available port number for us
        */
        let req_socket = UdpSocket::bind(format!(
            "{}:{}",
            CONFIG.host.hostname, 
            0
        ));

        if req_socket.is_err() {
            return Err(
                ResponseCode::ServerFailure
            );
        }

        match Self::query_rootserver(&mut self, &mut req_socket.unwrap()).await {
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

    async fn query_rootserver(&mut self, socket: &mut UdpSocket) -> Result<(), ResponseCode> {
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

        let mut root_s_datagram = DNS::new();
        root_s_datagram.header.qr = Type::Query;
        root_s_datagram.header.question_count = 1;
        root_s_datagram.header.op_code = OpCode::Query;
        root_s_datagram.header.id = 1;
        root_s_datagram.questions = vec![DNSQuestion { 
            name: question.name.clone(), 
            qtype: question.qtype, 
            class: question.class 
        }];

        let hostname = r_inst.split(" ")
            .into_iter()
            .next()
            .unwrap()
            .split("_")
            .nth(2)
            .unwrap();
        println!("{}",  format!("{}:{}", hostname, 53));

        socket.connect(format!("{}:{}", hostname, "53")).unwrap();
        socket.send_to(
            &*root_s_datagram.bytes().unwrap(),
            format!("{}:{}", hostname, "53")
        ).expect("Bruh fail!");

        let mut buf: [u8; 512] = [1; 512];
        socket.peek_from(&mut buf).expect("Failed");

        println!("{:?}", buf);

        todo!();
    }
}
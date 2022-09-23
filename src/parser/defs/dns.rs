use super::{
    header::DNSHeader, 
    question::DNSQuestion, 
    resource::DNSResourceFormat, 
    rcode::ResponseCode, 
    opcode::OpCode, 
    r#type::Type
};
use crate::parser::parse::{
    parse_datagram,
    datagram_bytes
};

#[derive(Debug)]
pub struct DNS {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answer: Option<DNSResourceFormat>,
    pub authority: Option<DNSResourceFormat>,
    pub additional: Option<DNSResourceFormat>
}

pub trait Construct {
    fn new() -> DNS;
    fn from(bytes: &[u8]) -> Result<DNS, ResponseCode>;
    fn bytes(self) -> Result<Vec<u8>, ResponseCode>;
}

impl Construct for DNS {
    fn from(bytes: &[u8]) -> Result<DNS, ResponseCode> {
        parse_datagram(bytes)
    }

    fn bytes(self) -> Result<Vec<u8>, ResponseCode> {
        datagram_bytes(self)
    }

    fn new() -> DNS {
        DNS { 
            header: DNSHeader { 
                id: 0, 
                qr: Type::Query, 
                op_code: OpCode::Status, 
                authoritative: false, 
                truncated: false, 
                recursion_desired: false, 
                recursion_available: false, 
                error_code: ResponseCode::NoError, 
                question_count: 0, 
                answer_count: 0, 
                nameserver_count: 0, 
                resource_count: 0
            }, 
            questions: vec![],
            answer: None,
            authority: None,
            additional: None
        }
    }
}
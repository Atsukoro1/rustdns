use bitreader::BitReader;

use super::{
    qclass::QuestionClass,
    qtype::QuestionType, 
    rcode::ResponseCode
};

#[derive(Debug, Clone)]
pub struct DNSResourceFormat {
    pub name: String,
    pub rr_type: QuestionType,
    pub rr_class: QuestionClass,
    pub ttl: u32,
    pub length: u16,
    pub data: String,
}

impl DNSResourceFormat {
    pub fn from(reader: &mut BitReader, bytes: &[u8]) -> Result<Self, ResponseCode> {
        Ok(DNSResourceFormat {
            name: String::from("test"),
            rr_class: QuestionClass::CH,
            rr_type: QuestionType::A,
            ttl: 483943,
            length: 32,
            data: String::from("fdlh")
        })
    }
}
use bitreader::BitReader;

use crate::resolver::nameresolver::resolve_name;

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
        let final_name = resolve_name(reader, bytes);

        reader.read_u16(16).unwrap();
        reader.read_u16(16).unwrap();

        let ttl = reader.read_u32(32).unwrap();
        let rdlength = reader.read_u16(16).unwrap();

        println!("{}", final_name);


        Ok(DNSResourceFormat {
            name: final_name,
            rr_class: QuestionClass::CH,
            rr_type: QuestionType::AFSDB,
            ttl: 483943,
            length: 32,
            data: String::from("fdlh")
        })
    }
}
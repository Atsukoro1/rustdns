use bitreader::BitReader;
use super::{
    header::DNSHeader, 
    question::DNSQuestion, 
    resource::DNSResourceFormat, 
    rcode::ResponseCode, 
    opcode::OpCode, 
    r#type::Type
};

#[derive(Debug)]
pub struct DNS {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answer: Option<DNSResourceFormat>,
    pub authority: Option<DNSResourceFormat>,
    pub additional: Option<DNSResourceFormat>
}

impl DNS {
    pub fn new() -> DNS {
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

    pub fn from(bytes: &[u8]) -> Result<DNS, ResponseCode> {
        let mut reader = BitReader::new(bytes);
        let result = DNSHeader::try_from(&mut reader)
            .unwrap();

        let answer: Option<DNSResourceFormat> = None;
        let authority: Option<DNSResourceFormat> = None;
        let additional: Option<DNSResourceFormat> = None;

        let questions = DNSQuestion::try_from(&mut reader, result.question_count)
            .unwrap();

        Ok(DNS {
            header: result,
            questions: questions,
            answer: answer,
            authority: authority,
            additional: additional
        })
    }

    pub fn bytes(self) -> Result<Vec<u8>, ResponseCode> {
        let mut bytes: Vec<u8> = vec![];

        DNSHeader::bytes(&mut bytes, &self);
        DNSQuestion::bytes(&mut bytes, &self);

        Ok(bytes)
    }
}
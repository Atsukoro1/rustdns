use bitreader::BitReader;
use crate::resolver::transport::TransportProto;

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
    pub questions: Option<Vec<DNSQuestion>>,
    pub answer: Option<Vec<DNSResourceFormat>>,
    pub authority: Option<Vec<DNSResourceFormat>>,
    pub additional: Option<Vec<DNSResourceFormat>>
}

impl DNS {
    pub fn new() -> DNS {
        DNS { 
            header: DNSHeader { 
                length: None,
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
                authority_count: 0, 
                additional_count: 0
            }, 
            questions: None,
            answer: None,
            authority: None,
            additional: None
        }
    }

    pub fn from(bytes: &[u8], proto: TransportProto) -> Result<DNS, ResponseCode> {
        let mut reader = BitReader::new(bytes);
        let result = DNSHeader::try_from(&mut reader, proto)
            .unwrap();

        let mut questions = None;
        let mut answer = None;
        let mut authority = None;
        let mut additional = None;

        if !result.truncated {
            questions = Some(
                DNSQuestion::try_from(&mut reader, result.question_count)
                    .unwrap()
            );

            // answer = if result.answer_count > 0 {
            //     let mut res: Vec<DNSResourceFormat> = vec![];
    
            //     for _ in 0..result.answer_count {
            //         res.push(
            //             DNSResourceFormat::from(&mut reader, bytes)
            //                 .unwrap()
            //         );
            //     }
    
            //     Some(res)
            // } else {
            //     None
            // };
    
            authority = if result.authority_count > 0 {
                let mut res: Vec<DNSResourceFormat> = vec![];
    
                for _ in 0..result.authority_count {
                    res.push(
                        DNSResourceFormat::from(&mut reader, bytes)
                            .unwrap()
                    );
                }
    
                Some(res)
            } else {
                None
            };
    
            additional = if result.additional_count > 0 {
                let mut res: Vec<DNSResourceFormat> = vec![];
    
                for _ in 0..result.additional_count {
                    res.push(
                        DNSResourceFormat::from(&mut reader, bytes)
                            .unwrap()
                    );
                }
    
                Some(res)
            } else {
                None
            };
        }

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
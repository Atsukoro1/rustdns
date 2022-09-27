use bitreader::BitReader;
use enum_primitive::FromPrimitive;
use crate::{
    convert_u16_to_two_u8s, 
    push_byte_vec
};

use super::{
    qclass::QuestionClass,
    qtype::QuestionType,
    rcode::ResponseCode, dns::DNS, 
    fqdn::FQDN
};

#[derive(Debug, Clone)]
pub struct DNSQuestion {
    pub name: FQDN,
    pub qtype: QuestionType,
    pub class: QuestionClass
}

impl DNSQuestion {
    pub fn try_from(reader: &mut BitReader, count: u16) -> Result<Vec<Self>, ResponseCode> {
        let mut questions: Vec<Self> = vec![];
        for _ in 0..count {
            let mut question: DNSQuestion = DNSQuestion { 
                name: FQDN::new(), 
                qtype: QuestionType::A,
                class: QuestionClass::CH
            };

            let mut qname: String = String::new();

            loop {
                let bytes_to_read: u8 = reader.read_u8(8).unwrap();

                // Separating byte will end the loop
                if bytes_to_read == 0 {
                    break;
                } 

                if qname.len() != 0 {
                    qname.push('.');
                }

                for _ in 0..bytes_to_read {
                    qname.push(std::char::from_u32(
                        reader.read_u32(8)
                            .unwrap()
                        )
                        .unwrap()
                    );
                }
            }

            question.name = FQDN::try_from(qname)
                .unwrap();
            question.qtype = QuestionType::from_u16(
                reader.read_u16(16).unwrap()
            ).unwrap();
            question.class = QuestionClass::from_u16(
                reader.read_u16(16).unwrap()
            ).unwrap();

            questions.push(question);
        };

        Ok(questions)
    }

    pub fn bytes(bytes: &mut Vec<u8>, datagram: &DNS) {
        let mut offset: u128 = 12;
        for question in datagram.questions.as_ref().unwrap() {
            let name_parts = question.name.split();

            let complete_length = question.name.len() + 6;
            push_byte_vec!(bytes, complete_length as u8 + 1, 0x0);

            // First write the initial length byte and then the content
            for name in name_parts {
                let length: usize = name.len();

                bytes[offset as usize] = length as u8;
                offset += 1;

                name.as_bytes()
                    .into_iter()
                    .for_each(|byte: &u8| {
                        bytes[offset as usize] = *byte; 
                        offset += 1;
                    });
            }

            // Terminating octet to make it clean that this is the end of domain name
            bytes[offset as usize] = 0x00; 
            offset += 1;

            let qtype_bytes: [u8; 2] = convert_u16_to_two_u8s!(question.qtype as u16, u16);

            bytes[offset as usize] = qtype_bytes[0];
            offset += 1;
            bytes[offset as usize] = qtype_bytes[1];
            offset += 1;

            let qclass_bytes: [u8; 2] = convert_u16_to_two_u8s!(question.class as u16, u16);

            bytes[offset as usize] = qclass_bytes[0];
            offset += 1;
            bytes[offset as usize] = qclass_bytes[1];
            offset += 1;
        };
    }
}
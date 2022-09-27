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
    pub fn from(reader: &mut BitReader) -> Result<Self, ResponseCode> {
        let mut final_res_name: String = String::from("");

        loop {
            let bytes_to_read: u8 = reader.read_u8(8).unwrap();

            // Separating byte will end the loop
            if bytes_to_read == 0 {
                break;
            } 

            if final_res_name.len() != 0 {
                final_res_name.push('.');
            }

            for _ in 0..bytes_to_read {
                final_res_name.push(std::char::from_u32(
                    reader.read_u32(8)
                        .unwrap()
                    )
                    .unwrap()
                );
            }
        }

        println!("{}", final_res_name);

        reader.read_u16(16).unwrap();
        reader.read_u16(16).unwrap();

        let ttl = reader.read_u32(32).unwrap();
        let rdlength = reader.read_u16(16).unwrap();

        reader.skip((rdlength).into());

        Ok(DNSResourceFormat {
            name: final_res_name,
            rr_class: QuestionClass::CH,
            rr_type: QuestionType::AFSDB,
            ttl: ttl,
            length: rdlength,
            data: String::from("fdlh")
        })
    }
}
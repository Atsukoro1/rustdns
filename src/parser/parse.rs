use bitreader::BitReader;
use crate::{
    helper::bit_assign,
    parser::def::{
        DNS,
        DNSHeader,
        OpCode,
        Type,
        ErrorCode,
        DNSQuestion
    }
};

/// Parse raw bytes into DNS struct
/// 
/// As described at https://datatracker.ietf.org/doc/html/rfc1035 under
/// the 4.1.1 Header section format
/// 
/// Returns DNS struct containing header and question
pub fn parse_datagram(bytes: &[u8]) -> DNS {
    let mut reader = BitReader::new(bytes);
    let mut result = DNSHeader {
        id: 0,
        qr: Type::Query,
        op_code: OpCode::FutureUse,
        authoritative: false,
        truncated: false,
        recursion_desired: false,
        recursion_available: false,
        error_code: ErrorCode::NotImplemented,
        question_count: 0,
        answer_count: 0,
        nameserver_count: 0,
        resource_count: 0
    };

    result.id = reader.read_u16(16).unwrap();

    result.qr = bit_assign::<Type>(
        Type::Query, 
        Type::Response,
        &mut reader
    );

    result.op_code = match reader.read_u8(4).unwrap() {
        0 => OpCode::Query,
        1 => OpCode::IQuery,
        2 => OpCode::Status,
        _ => OpCode::FutureUse
    };

    result.authoritative = bit_assign::<bool>(
        false, 
        true, 
        &mut reader
    );

    result.truncated = bit_assign::<bool>(
        false, 
        true, 
        &mut reader
    );

    result.recursion_desired = bit_assign::<bool>(
        false, 
        true, 
        &mut reader
    );

    result.recursion_available = bit_assign::<bool>(
        false, 
        true, 
        &mut reader
    );

    reader.skip(3).unwrap();

    result.error_code = match reader.read_u8(4).unwrap() {
        0 => ErrorCode::NoError,
        1 => ErrorCode::FormatError,
        2 => ErrorCode::ServerFailure,
        3 => ErrorCode::NameError,
        4 => ErrorCode::NotImplemented,
        5 => ErrorCode::Refused,
        _ => ErrorCode::FutureUse
    };

    result.question_count = reader.read_u16(16).unwrap();
    result.answer_count = reader.read_u16(16).unwrap();
    result.nameserver_count = reader.read_u16(16).unwrap();
    result.resource_count = reader.read_u16(16).unwrap();

    let mut questions: Vec<DNSQuestion> = vec![];
    for _ in 0..result.question_count {
        let mut question: DNSQuestion = DNSQuestion { 
            name: String::new(), 
            qtype: 1,
            class: 1 
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

        question.name = qname;

        questions.push(question);
    }

    DNS {
        header: result,
        questions: questions
    }
}
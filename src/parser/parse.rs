use bitreader::BitReader;
use bitbit::BitWriter;
use enum_primitive::FromPrimitive;
use crate::{
    helper::bit_assign,
    parser::def::{
        DNS,
        DNSHeader,
        OpCode,
        QuestionClass,
        QuestionType,
        Type,
        ErrorCode,
        DNSQuestion
    }
};

/// Convert DNS struct into raw bytes
pub fn datagram_bytes(datagram: DNS) -> Box<[u16]> {
    let mut result = [1u16; 520];

    result[0] = datagram.header.id;

    let mut option_bytes: u16 = 0b10000000_00000000;
    option_bytes |= 0b001;
    
    println!("0b{:016b}", option_bytes);
    
    Box::from(result)
}

/// Create a DNS struct from raw bytes
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

        question.name = qname;
        question.qtype = QuestionType::from_u16(
            reader.read_u16(16).unwrap()
        ).unwrap();
        question.class = QuestionClass::from_u16(
            reader.read_u16(16).unwrap()
        ).unwrap();

        questions.push(question);
    }

    DNS {
        header: result,
        questions: questions
    }
}
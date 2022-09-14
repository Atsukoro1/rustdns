use bit::BitIndex;
use enum_primitive::FromPrimitive;
use bitreader::BitReader;
use crate::{
    helpers::bit::{
        convert_u16_to_two_u8s,
        bit_assign
    },
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
pub fn datagram_bytes(datagram: DNS) -> Box<[u8]> {
    let mut bytes: [u8; 12] = [
        0b0000_0000u8, 0b0000_0000u8,
        0b0000_0000u8, 0b0000_0000u8,
        0b0000_0000u8, 0b0000_0000u8,
        0b0000_0000u8, 0b0000_0000u8,
        0b0000_0000u8, 0b0000_0000u8,
        0b0000_0000u8, 0b0000_0000u8
    ];

    // Identificator
    let id_u8: [u8; 2] = convert_u16_to_two_u8s(datagram.header.id);
    bytes[0].set_bit_range(0..7, id_u8[0]);
    bytes[1].set_bit_range(0..7, id_u8[1]);

    // Question or response
    bytes[2].set_bit(
        0, 
        if datagram.header.qr == Type::Query {
            false
        } else {
            true
        },
    );

    // Opcode
    let opc_bits: u8 = match datagram.header.op_code {
        OpCode::Query => 0x1,
        OpCode::IQuery => 0x2,
        OpCode::Status => 0x3,
        OpCode::FutureUse => 0x4
    };
    bytes[2].set_bit_range(1..4, opc_bits);

    // Authoritative Answer
    bytes[2].set_bit(
        5, 
        datagram.header.authoritative
    );

    // If the message was truncated
    bytes[2].set_bit(
        6, 
        datagram.header.truncated
    );

    // If recursion will be used
    bytes[2].set_bit(
        7, 
        datagram.header.recursion_desired
    );

    // If recursion is available
    bytes[3].set_bit(
        0, 
        datagram.header.recursion_available
    );

    // Skipped 3 bits because of the section that will be used in future
    let rcode_bits: u8 = match datagram.header.error_code {
        ErrorCode::NoError => 0x0,
        ErrorCode::FormatError => 0x1,
        ErrorCode::ServerFailure => 0x2,
        ErrorCode::NameError => 0x3,
        ErrorCode::NotImplemented => 0x4,
        ErrorCode::Refused => 0x5,
        // Future use
        _ => 0x0
    };
    bytes[3].set_bit_range(4..7, rcode_bits);

    let q_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.question_count);
    bytes[4].set_bit_range(0..7, q_bits[0]);
    bytes[5].set_bit_range(0..7, q_bits[1]);

    let an_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.answer_count);
    bytes[6].set_bit_range(0..7, an_bits[0]);
    bytes[7].set_bit_range(0..7, an_bits[1]);

    let ns_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.nameserver_count);
    bytes[8].set_bit_range(0..7, ns_bits[0]);
    bytes[9].set_bit_range(0..7, ns_bits[1]);

    let ar_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.resource_count);
    bytes[10].set_bit_range(0..7, ar_bits[0]);
    bytes[11].set_bit_range(0..7, ar_bits[1]);

    Box::from(bytes)
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
use enum_primitive::FromPrimitive;
use bitreader::BitReader;
use bit::BitIndex;
use crate::{
    helpers::bit::{
        convert_u16_to_two_u8s,
        bit_assign, push_byte_vec
    },
    parser::defs::{
        dns::DNS,
        header::DNSHeader,
        opcode::OpCode,
        qclass::QuestionClass,
        qtype::QuestionType,
        r#type::Type,
        resource::DNSResourceFormat,
        rcode::ResponseCode,
        question::DNSQuestion
    }
};

/// Convert DNS struct into raw bytes
pub fn datagram_bytes(datagram: DNS) -> Result<Vec<u8>, ResponseCode> {
    // Pre-fill first 12 bytes for the header
    let mut bytes: Vec<u8> = vec![];

    // Identificator
    push_byte_vec(&mut bytes, 2, 0x0);
    let id_u8: [u8; 2] = convert_u16_to_two_u8s(datagram.header.id);
    bytes[0] = id_u8[0];
    bytes[1] = id_u8[1];

    // Question or response
    push_byte_vec(&mut bytes, 1, 0x0);
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
        OpCode::Query => 0x0,
        OpCode::IQuery => 0x1,
        OpCode::Status => 0x2
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
    push_byte_vec(&mut bytes, 1, 0x0);
    bytes[3].set_bit(
        0, 
        datagram.header.recursion_available
    );

    // Skipped 3 bits because of the section that will be used in future
    let rcode_bits: u8 = match datagram.header.error_code {
        ResponseCode::NoError => 0x0,
        ResponseCode::FormatError => 0x1,
        ResponseCode::ServerFailure => 0x2,
        ResponseCode::NameError => 0x3,
        ResponseCode::NotImplemented => 0x4,
        ResponseCode::Refused => 0x5,
    };
    bytes[3].set_bit_range(4..7, rcode_bits);

    push_byte_vec(&mut bytes, 2, 0x0);
    let q_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.question_count);
    bytes[4].set_bit_range(0..7, q_bits[0]);
    bytes[5].set_bit_range(0..7, q_bits[1]);

    push_byte_vec(&mut bytes, 2, 0x0);
    let an_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.answer_count);
    bytes[6].set_bit_range(0..7, an_bits[0]);
    bytes[7].set_bit_range(0..7, an_bits[1]);

    push_byte_vec(&mut bytes, 2, 0x0);
    let ns_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.nameserver_count);
    bytes[8].set_bit_range(0..7, ns_bits[0]);
    bytes[9].set_bit_range(0..7, ns_bits[1]);

    push_byte_vec(&mut bytes, 2, 0x0);
    let ar_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.resource_count);
    bytes[10].set_bit_range(0..7, ar_bits[0]);
    bytes[11].set_bit_range(0..7, ar_bits[1]);

    let mut offset: u128 = 12;
    for question in datagram.questions {
        let name_parts = question.name.split(".")
            .collect::<Vec<&str>>();

        let complete_length = question.name.len() + 6;
        push_byte_vec(&mut bytes, complete_length as u8, 0x0);

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

        let qtype_bytes: [u8; 2] = convert_u16_to_two_u8s(question.qtype as u16);

        bytes[offset as usize] = qtype_bytes[0];
        offset += 1;
        bytes[offset as usize] = qtype_bytes[1];
        offset += 1;

        let qclass_bytes: [u8; 2] = convert_u16_to_two_u8s(question.class as u16);
        
        bytes[offset as usize] = qclass_bytes[0];
        offset += 1;
        bytes[offset as usize] = qclass_bytes[1];
        offset += 1;
    };

    Ok(bytes)
}

/// Create a DNS struct from raw bytes
pub fn parse_datagram(bytes: &[u8]) -> Result<DNS, ResponseCode> {
    let mut reader = BitReader::new(bytes);
    let mut result = DNSHeader {
        id: 0,
        qr: Type::Query,
        op_code: OpCode::IQuery,
        authoritative: false,
        truncated: false,
        recursion_desired: false,
        recursion_available: false,
        error_code: ResponseCode::NotImplemented,
        question_count: 0,
        answer_count: 0,
        nameserver_count: 0,
        resource_count: 0
    };

    let answer: Option<DNSResourceFormat> = None;
    let authority: Option<DNSResourceFormat> = None;
    let additional: Option<DNSResourceFormat> = None;

    result.id = reader.read_u16(16).unwrap();

    result.qr = bit_assign::<Type>(
        Type::Query, 
        Type::Response,
        &mut reader
    );

    result.op_code = OpCode::from_u8(reader.read_u8(4).unwrap())
        .unwrap();

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

    result.error_code = ResponseCode::from_u8(
        reader.read_u8(4).unwrap()
    ).unwrap();

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

    Ok(DNS {
        header: result,
        questions: questions,
        answer: answer,
        authority: authority,
        additional: additional
    })
}
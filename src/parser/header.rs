use bit::BitIndex;
use bitreader::BitReader;
use enum_primitive::FromPrimitive;
use crate::helpers::bit::{bit_assign, push_byte_vec, convert_u16_to_two_u8s};

use super::{
    r#type::Type, 
    opcode::OpCode, 
    rcode::ResponseCode, dns::DNS
};

#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub qr: Type,
    pub op_code: OpCode,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub error_code: ResponseCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub nameserver_count: u16,
    pub resource_count: u16
}

impl DNSHeader {
    pub fn try_from(reader: &mut BitReader) -> Result<Self, ResponseCode> {
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

        result.id = reader.read_u16(16).unwrap();

        result.qr = bit_assign::<Type>(
            Type::Query, 
            Type::Response,
            reader
        );

        result.op_code = OpCode::from_u8(reader.read_u8(4).unwrap())
            .unwrap();

        result.authoritative = bit_assign::<bool>(
            false, 
            true, 
            reader
        );

        result.truncated = bit_assign::<bool>(
            false, 
            true, 
            reader
        );

        result.recursion_desired = bit_assign::<bool>(
            false, 
            true, 
            reader
        );

        result.recursion_available = bit_assign::<bool>(
            false, 
            true, 
            reader
        );

        reader.skip(3).unwrap();

        result.error_code = ResponseCode::from_u8(
            reader.read_u8(4).unwrap()
        ).unwrap();

        result.question_count = reader.read_u16(16).unwrap();
        result.answer_count = reader.read_u16(16).unwrap();
        result.nameserver_count = reader.read_u16(16).unwrap();
        result.resource_count = reader.read_u16(16).unwrap();
        
        Ok(result)
    }

    pub fn bytes(bytes: &mut Vec<u8>, datagram: &DNS) {
        push_byte_vec(bytes, 2, 0x0);
        let id_u8: [u8; 2] = convert_u16_to_two_u8s(datagram.header.id);
        bytes[0] = id_u8[0];
        bytes[1] = id_u8[1];
        
        // Question or response
        push_byte_vec(bytes, 1, 0x0);
        bytes[2].set_bit(
            0, 
            if datagram.header.qr == Type::Query {
                false
            } else {
                true
            },
        );
    
        // Opcode
        let opc_bits: u8 = datagram.header.op_code.try_into()
            .unwrap();
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
        push_byte_vec(bytes, 1, 0x0);
        bytes[3].set_bit(
            0, 
            datagram.header.recursion_available
        );
    
        // Skipped 3 bits because of the section that will be used in future
        let rcode_bits: u8 = datagram.header.error_code.try_into()
            .unwrap();
        bytes[3].set_bit_range(4..7, rcode_bits);
    
        push_byte_vec(bytes, 2, 0x0);
        let q_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.question_count);
        bytes[4].set_bit_range(0..7, q_bits[0]);
        bytes[5].set_bit_range(0..7, q_bits[1]);
    
        push_byte_vec(bytes, 2, 0x0);
        let an_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.answer_count);
        bytes[6].set_bit_range(0..7, an_bits[0]);
        bytes[7].set_bit_range(0..7, an_bits[1]);
    
        push_byte_vec(bytes, 2, 0x0);
        let ns_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.nameserver_count);
        bytes[8].set_bit_range(0..7, ns_bits[0]);
        bytes[9].set_bit_range(0..7, ns_bits[1]);
    
        push_byte_vec(bytes, 2, 0x0);
        let ar_bits: [u8; 2] = convert_u16_to_two_u8s(datagram.header.resource_count);
        bytes[10].set_bit_range(0..7, ar_bits[0]);
        bytes[11].set_bit_range(0..7, ar_bits[1]);
    }
}
use std::fs::read;

use bitreader::BitReader;
use super::helper::bit_assign;

#[derive(Debug)]
pub enum Type {
    Response,
    Query
}

#[derive(Debug)]
pub enum OpCode {
    /// a standard query (QUERY)
    Query,
    /// an inverse query (IQUERY)
    IQuery,
    /// a server status request (STATUS)
    Status,
    /// reserved for future use
    FutureUse
}

#[derive(Debug)]
pub enum ErrorCode {
    /// No error condition
    NoError,
    /// Format error - The name server was unable to interpret the query.
    FormatError,
    /// Server failure - The name server was unable to process
    /// this query due to a problem with the name server.
    ServerFailure,
    /// Meaningful only for responses from an authoritative name
    /// server, this code signifies that the domain name referenced in the query does
    /// not exist.
    NameError,
    /// The name server does not support the requested kind of query.
    NotImplemented,
    /// The name server refuses to perform the specified operation for
    /// policy reasons.  For example, a nameserver may not wish to provide the
    /// information to the particular requester, or a name server may not wish to perform
    /// a particular operation (e.g., zonetransfer) for particular data.
    Refused,
    /// Reserved for future use
    FutureUse
}

/// DNS Header as described at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1.1. Header section format
#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub qr: Type,
    pub op_code: OpCode,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub error_code: ErrorCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub nameserver_count: u16,
    pub resource_count: u16
}

/// Parse raw bytes into DNS struct
/// 
/// As described at https://datatracker.ietf.org/doc/html/rfc1035 under
/// the 4.1.1 Header section format
/// 
/// Returns DNS struct
pub fn parse_datagram(bytes: &[u8]) -> DNSHeader {
    println!("{}", bytes.len());
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

    reader.skip(3);


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

    println!("{}",reader.remaining());

    result
}
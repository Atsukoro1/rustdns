use super::{r#type::Type, opcode::OpCode, rcode::ResponseCode};

/// DNS Header as described at https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
/// under the 4.1.1. Header section format
#[derive(Debug)]
pub struct DNSHeader {
    /// A 16 bit identifier assigned by the program that
    /// generates any kind of query.  This identifier is copied
    /// the corresponding reply and can be used by the requester
    /// to match up replies to outstanding queries.
    pub id: u16,

    /// A one bit field that specifies whether this message is a
    /// query (0), or a response (1).
    pub qr: Type,

    /// A four bit field that specifies kind of query in this
    /// message.  This value is set by the originator of a query
    /// and copied into the response. These values are further
    /// described in OpCode struct definition
    pub op_code: OpCode,

    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    pub authoritative: bool,

    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    /// Note that the contents of the answer section may have
    /// multiple owner names because of aliases.  The AA bit
    /// corresponds to the name which matches the query name, or
    /// the first owner name in the answer section.
    pub truncated: bool,

    /// Recursion Desired - this bit may be set in a query and
    /// is copied into the response.  If RD is set, it directs
    /// the name server to pursue the query recursively.
    /// Recursive query support is optional.
    pub recursion_desired: bool,

    /// Recursion Available - this be is set or cleared in a
    /// response, and denotes whether recursive query support is
    /// available in the name server.
    pub recursion_available: bool,

    /// Response code - this 4 bit field is set as part of
    /// responses. The values are further described in ErrorCode
    /// struct definition
    pub error_code: ResponseCode,

    /// an unsigned 16 bit integer specifying the number of
    /// entries in the question section.
    pub question_count: u16,

    /// an unsigned 16 bit integer specifying the number of
    /// resource records in the answer section.
    pub answer_count: u16,

    /// an unsigned 16 bit integer specifying the number of name
    /// server resource records in the authority records
    /// section.
    pub nameserver_count: u16,

    /// an unsigned 16 bit integer specifying the number of
    /// resource records in the additional records section.
    pub resource_count: u16
}
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

/// DNS Question as described at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1.2. Question section format section
#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub qtype: u16,
    pub class: u16
}

/// DNS datagram as descibed at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1. Format section
#[derive(Debug)]
pub struct DNS {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>
}
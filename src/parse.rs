pub enum Type {
    Response,
    Query
}

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
    Refused
}

/// DNS Header as described at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1.1. Header section format
pub struct DNSHeader {
    id: &'static [u8],
    qr: Type,
    op: OpCode,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    error_code: ErrorCode,
    question_count: u16,
    answer_count: u16,
    nameserver_count: u16,
    resource_count: u16
}

/// Parse raw bytes into DNS struct
/// 
/// As described at https://datatracker.ietf.org/doc/html/rfc1035 under
/// the 4.1.1 Header section format
/// 
/// Returns DNS struct
pub fn parse_datagram() {
}
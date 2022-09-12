use enum_primitive::FromPrimitive;

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
    pub qtype: QuestionType,
    pub class: u16
}

enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    /// This list of DNS record types is an overview of resource records (RRs) 
    /// permissible in zone files of the Domain Name System (DNS). It also contains pseudo-RRs. 
    /// 
    /// This list does not list records with name containing non-alphanumerical characters
    /// These records are listed here and should be checked independently
    /// * -> All cached records (255)
    /// NSAP-PTR -> Not in current use by any notable application (23)
    /// 
    /// Described at https://en.wikipedia.org/wiki/List_of_DNS_record_types
    pub enum QuestionType {
        A = 1,
        NS = 2,
        MD = 3,
        MF = 4,
        CNAME = 5,
        SOA = 6,
        MB = 7,
        MG = 8,
        MR = 9, 
        NULL = 10,
        WKS = 11,
        HINFO = 13,
        MINFO = 14,
        MX = 15,
        TXT = 16,
        RP = 17,
        AFSDB = 18,
        X25 = 19,
        ISDN = 20,
        RT = 21,
        NSAP = 22,
        SIG = 24,
        KEY = 25,
        PX = 26,
        GPOS = 27,
        AAAA = 28,
        LOC = 29,
        NXT = 30,
        EID = 31,
        NB = 32,
        NBSTAT = 33,
        ATMA = 34,
        NAPTR = 35,
        KX = 36,
        CERT = 37,
        DNAME = 39,
        OPTION = 41,
        APL = 42,
        DS = 43,
        SSHFP = 44,
        IPSECKEY = 45,
        RRSIG = 46,
        NSEC = 47,
        DNSKEY = 48,
        DHCID = 49,
        NSEC3 = 50,
        NSEC3PARAM = 51,
        TLSA = 52,
        SMIMEA = 53,
        HIP = 55,
        NINFO = 56,
        RKEY = 57,
        TALINK = 58,
        CDS = 59,
        CDNSKEY = 60,
        OPENPGPKEY = 61,
        CSYNC = 62,
        ZONEMD = 63,
        SVCB = 64,
        HTTPS = 65,
        SPF = 99,
        UINFO = 100,
        UID = 101,
        GID = 102,
        UNSPEC = 103,
        NID = 104,
        L32 = 105,
        L64 = 106,
        LP = 107,
        EUI48 = 108,
        EUI64 = 109,
        TKEY = 249,
        TSIG = 250,
        IXFR = 251,
        AXFR = 252,
        MAILB = 253,
        MAILA = 254,
    }
}

/// DNS datagram as descibed at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1. Format section
#[derive(Debug)]
pub struct DNS {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>
}
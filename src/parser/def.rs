use crate::parser::parse::parse_datagram;

use super::parse::datagram_bytes;

#[derive(Debug, PartialEq)]
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
    Refused
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
        PTR = 12,
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
    pub error_code: ErrorCode,

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

/// DNS Question as described at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1.2. Question section format section
#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub qtype: QuestionType,
    pub class: QuestionClass
}

#[derive(Debug)]
/// DNS Resource format, can be answer, authority or additional
/// As specified at https://www.ietf.org/rfc/rfc1035.html#section-4.1.3 under the
/// Resource record format section
pub struct DNSResourceFormat {
    /// a domain name to which this resource record pertains.
    pub name: String,

    /// two octets containing one of the RR type codes.  This
    /// field specifies the meaning of the data in the RDATA
    /// field.
    pub rr_type: QuestionType,

    /// two octets which specify the class of the data in the
    /// RDATA field.
    pub rr_class: QuestionClass,

    /// a 32 bit unsigned integer that specifies the time
    /// interval (in seconds) that the resource record may be
    /// cached before it should be discarded.  Zero values are
    /// interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.
    pub ttl: u32,

    /// an unsigned 16 bit integer that specifies the length in
    /// octets of the RDATA field.
    pub length: u16,

    /// a variable length string of octets that describes the
    /// resource.  The format of this information varies
    /// according to the TYPE and CLASS of the resource record.
    /// For example, the if the TYPE is A and the CLASS is IN,
    /// the RDATA field is a 4 octet ARPA Internet address.
    pub data: String,
}

enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    /// DNS Question class
    /// described at https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
    pub enum QuestionClass {
        /// the Internet
        IN = 1,

        /// the CSNET class (Obsolete - used only for examples in
        /// some obsolete RFCs)
        CS = 2,

        /// the CHAOS class
        CH = 3,

        /// Hesiod [Dyer 87]
        HS = 4
    }
}

/// DNS datagram as descibed at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1. Format section
#[derive(Debug)]
pub struct DNS {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answer: Option<DNSResourceFormat>,
    pub authority: Option<DNSResourceFormat>,
    pub additional: Option<DNSResourceFormat>
}

pub trait Construct {
    /// Create a new DNS struct with some default values
    fn new() -> DNS;

    /// Parse raw bytes into DNS struct
    /// 
    /// As described at https://datatracker.ietf.org/doc/html/rfc1035 under
    /// the 4.1.1 Header section format
    /// 
    /// Returns a Result that can contain DNS struct or an error code with 
    /// specific error that will be contained in invalid response
    fn from(bytes: &[u8]) -> Result<DNS, ErrorCode>;

    /// Convert the struct into bytes
    fn bytes(self) -> Result<Vec<u8>, ErrorCode>;
}

impl Construct for DNS {
    fn new() -> DNS {
        DNS { 
            header: DNSHeader { 
                id: 0, 
                qr: Type::Query, 
                op_code: OpCode::FutureUse, 
                authoritative: false, 
                truncated: false, 
                recursion_desired: false, 
                recursion_available: false, 
                error_code: ErrorCode::NoError, 
                question_count: 0, 
                answer_count: 0, 
                nameserver_count: 0, 
                resource_count: 0
            }, 
            questions: vec![],
            answer: None,
            authority: None,
            additional: None
        }
    }

    fn from(bytes: &[u8]) -> Result<DNS, ErrorCode> {
        parse_datagram(bytes)
    }

    fn bytes(self) -> Result<Vec<u8>, ErrorCode> {
        datagram_bytes(self)
    }
}
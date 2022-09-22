use super::{
    qclass::QuestionClass,
    qtype::QuestionType
};

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
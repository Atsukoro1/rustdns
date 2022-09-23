/// https://www.ietf.org/rfc/rfc1035.html#section-4.1
pub mod dns;

/// https://www.ietf.org/rfc/rfc1035.html#section-4.1.1
pub mod header;
pub mod opcode;
pub mod rcode;
pub mod r#type;

/// https://www.ietf.org/rfc/rfc1035.html#section-4.1.2
pub mod qclass;
pub mod qtype;
pub mod question;

/// https://www.ietf.org/rfc/rfc1035.html#section-4.1.3
pub mod resource;
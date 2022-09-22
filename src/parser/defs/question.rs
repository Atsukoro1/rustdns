use super::{
    qclass::QuestionClass,
    qtype::QuestionType
};

/// DNS Question as described at https://datatracker.ietf.org/doc/html/rfc1035
/// under the 4.1.2. Question section format section
#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub qtype: QuestionType,
    pub class: QuestionClass
}
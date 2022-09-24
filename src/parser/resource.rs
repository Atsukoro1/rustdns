use super::{
    qclass::QuestionClass,
    qtype::QuestionType
};

#[derive(Debug, Clone)]
pub struct DNSResourceFormat {
    pub name: String,
    pub rr_type: QuestionType,
    pub rr_class: QuestionClass,
    pub ttl: u32,
    pub length: u16,
    pub data: String,
}
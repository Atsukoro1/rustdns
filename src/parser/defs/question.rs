use super::{
    qclass::QuestionClass,
    qtype::QuestionType
};

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub qtype: QuestionType,
    pub class: QuestionClass
}
enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum OpCode {
        Query = 0x0,
        IQuery = 0x1,
        Status = 0x2
    }
}
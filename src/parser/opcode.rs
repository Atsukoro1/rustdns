enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum OpCode {
        Query = 0x0,
        IQuery = 0x1,
        Status = 0x2
    }
}

impl TryInto<u8> for OpCode {
    type Error = String;

    fn try_into(self) -> Result<u8, Self::Error> {
        let res = match self {
            OpCode::Query => 0x3,
            OpCode::IQuery => 0x1,
            OpCode::Status => 0x2
        };

        Ok(res)
    }
}
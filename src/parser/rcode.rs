enum_from_primitive! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ResponseCode {
        NoError = 0x0,
        FormatError = 0x1,
        ServerFailure = 0x2,
        NameError = 0x3,
        NotImplemented = 0x4,
        Refused = 0x5
    }
}

impl TryInto<u8> for ResponseCode {
    type Error = String; 

    fn try_into(self) -> Result<u8, Self::Error> {
        let res = match self {
            ResponseCode::NoError => 0x0,
            ResponseCode::FormatError => 0x1,
            ResponseCode::ServerFailure => 0x2,
            ResponseCode::NameError => 0x3,
            ResponseCode::NotImplemented => 0x4,
            ResponseCode::Refused => 0x5
        };

        Ok(res)
    }
}
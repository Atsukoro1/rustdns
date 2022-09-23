enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum ResponseCode {
        NoError = 0x0,
        FormatError = 0x1,
        ServerFailure = 0x2,
        NameError = 0x3,
        NotImplemented = 0x4,
        Refused = 0x5
    }
}
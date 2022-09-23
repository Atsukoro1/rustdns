enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    #[repr(u8)]
    pub enum Type {
        Response = 0x1,
        Query = 0x0
    }
}
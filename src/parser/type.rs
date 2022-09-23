enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Type {
        Response = 0x1,
        Query = 0x0
    }
}
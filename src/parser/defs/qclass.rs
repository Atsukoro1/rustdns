enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    pub enum QuestionClass {
        IN = 0x1,
        CS = 0x2,
        CH = 0x3,
        HS = 0x4
    }
}
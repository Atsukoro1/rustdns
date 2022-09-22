enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    pub enum QuestionClass {
        IN = 1,
        CS = 2,
        CH = 3,
        HS = 4
    }
}
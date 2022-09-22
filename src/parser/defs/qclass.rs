enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    /// DNS Question class
    /// described at https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
    pub enum QuestionClass {
        /// the Internet
        IN = 1,

        /// the CSNET class (Obsolete - used only for examples in
        /// some obsolete RFCs)
        CS = 2,

        /// the CHAOS class
        CH = 3,

        /// Hesiod [Dyer 87]
        HS = 4
    }
}
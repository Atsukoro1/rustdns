enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    /// This list of DNS record types is an overview of resource records (RRs) 
    /// permissible in zone files of the Domain Name System (DNS). It also contains pseudo-RRs. 
    /// 
    /// This list does not list records with name containing non-alphanumerical characters
    /// These records are listed here and should be checked independently
    /// * -> All cached records (255)
    /// NSAP-PTR -> Not in current use by any notable application (23)
    /// 
    /// Described at https://en.wikipedia.org/wiki/List_of_DNS_record_types
    pub enum QuestionType {
        A = 1,
        NS = 2,
        MD = 3,
        MF = 4,
        CNAME = 5,
        SOA = 6,
        MB = 7,
        MG = 8,
        MR = 9, 
        NULL = 10,
        WKS = 11,
        PTR = 12,
        HINFO = 13,
        MINFO = 14,
        MX = 15,
        TXT = 16,
        RP = 17,
        AFSDB = 18,
        X25 = 19,
        ISDN = 20,
        RT = 21,
        NSAP = 22,
        SIG = 24,
        KEY = 25,
        PX = 26,
        GPOS = 27,
        AAAA = 28,
        LOC = 29,
        NXT = 30,
        EID = 31,
        NB = 32,
        NBSTAT = 33,
        ATMA = 34,
        NAPTR = 35,
        KX = 36,
        CERT = 37,
        DNAME = 39,
        OPTION = 41,
        APL = 42,
        DS = 43,
        SSHFP = 44,
        IPSECKEY = 45,
        RRSIG = 46,
        NSEC = 47,
        DNSKEY = 48,
        DHCID = 49,
        NSEC3 = 50,
        NSEC3PARAM = 51,
        TLSA = 52,
        SMIMEA = 53,
        HIP = 55,
        NINFO = 56,
        RKEY = 57,
        TALINK = 58,
        CDS = 59,
        CDNSKEY = 60,
        OPENPGPKEY = 61,
        CSYNC = 62,
        ZONEMD = 63,
        SVCB = 64,
        HTTPS = 65,
        SPF = 99,
        UINFO = 100,
        UID = 101,
        GID = 102,
        UNSPEC = 103,
        NID = 104,
        L32 = 105,
        L64 = 106,
        LP = 107,
        EUI48 = 108,
        EUI64 = 109,
        TKEY = 249,
        TSIG = 250,
        IXFR = 251,
        AXFR = 252,
        MAILB = 253,
        MAILA = 254,
    }
}
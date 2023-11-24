#![allow(clippy::upper_case_acronyms)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    /// a host address
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// a mail destination (Obsolete - use MX)
    MD = 3,
    /// a mail forwarder (Obsolete - use MX)
    MF = 4,
    /// the canonical name for an alias
    CNAME = 5,
    /// marks the start of a zone of authority
    SOA = 6,
    /// a mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// a mail group member (EXPERIMENTAL)
    MG = 8,
    /// a mail rename domain name (EXPERIMENTAL)
    MR = 9,
    ///  a null RR (EXPERIMENTAL)
    NULL = 10,
    ///  a well known service description
    WKS = 11,
    ///  a domain name pointer
    PTR = 12,
    ///  host information
    HINFO = 13,
    ///  mailbox or mail list information
    MINFO = 14,
    ///  mail exchange
    MX = 15,
    ///  text strings
    TXT = 16,
}
impl From<u16> for ResponseType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            _ => panic!("Unsupported value for ResponseType: {value:?}"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseClass {
    /// The Internet
    IN = 1,
    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,
    /// The CHAOS class
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,
}
impl From<u16> for ResponseClass {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => panic!("Unsupported value for ResponseClass: {value:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// a host address
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// a mail destination (Obsolete - use MX)
    MD = 3,
    /// a mail forwarder (Obsolete - use MX)
    MF = 4,
    /// the canonical name for an alias
    CNAME = 5,
    /// marks the start of a zone of authority
    SOA = 6,
    /// a mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// a mail group member (EXPERIMENTAL)
    MG = 8,
    /// a mail rename domain name (EXPERIMENTAL)
    MR = 9,
    ///  a null RR (EXPERIMENTAL)
    NULL = 10,
    ///  a well known service description
    WKS = 11,
    ///  a domain name pointer
    PTR = 12,
    ///  host information
    HINFO = 13,
    ///  mailbox or mail list information
    MINFO = 14,
    ///  mail exchange
    MX = 15,
    ///  text strings
    TXT = 16,
    /// A request for a transfer of an entire zone
    AXFR = 252,
    /// A request for mailbox-related records (MB, MG or MR)
    MAILB = 253,
    /// A request for mail agent RRs (Obsolete - see MX)
    MAILA = 254,
    /// A request for all records
    All = 255,
}
impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            252 => Self::AXFR,
            253 => Self::MAILB,
            254 => Self::MAILA,
            255 => Self::All,
            _ => panic!("Unsupported value for QueryType: {value:?}"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryClass {
    /// The Internet
    IN = 1,
    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,
    /// The CHAOS class
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,
    /// All class values
    All = 255,
}
impl From<u16> for QueryClass {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            255 => Self::All,
            _ => panic!("Unsupported value for QueryClass: {value:?}"),
        }
    }
}

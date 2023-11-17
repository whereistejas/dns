use arrayvec::ArrayVec;

use crate::constants::{QueryClass, QueryType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Query {
    /// Encoded domain name.
    qname: ArrayVec<u8, 255>,
    qtype: QueryType,
    qclass: QueryClass,
}

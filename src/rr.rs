use arrayvec::ArrayVec;

use crate::constants::{ResponseClass, ResponseType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResponseRecord {
    pub(crate) name: ArrayVec<u8, 255>,
    pub(crate) type_: ResponseType,
    pub(crate) class: ResponseClass,
    pub(crate) ttl: u32,
    pub(crate) rd_length: u16,
    pub(crate) rdata: ArrayVec<u8, 249>,
}

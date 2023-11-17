use arrayvec::ArrayVec;

use crate::constants::{ResponseClass, ResponseType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResponseRecord {
    pub(crate) name: ArrayVec<u8, 255>,
    pub(crate) type_: ResponseType,
    pub(crate) class: ResponseClass,
    pub(crate) ttl: u32,
    pub(crate) rd_length: u16,
    rdata: RData,
}
impl ResponseRecord {
    pub(crate) fn from_bytes(decoder: &mut crate::decoder::Decoder<'_>) -> _ {
        todo!()
            rdata: RData::from_bytes(type_, class, decoder)?,

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RData {
    CNAME,
    Other,
}

impl RData {
    fn from_bytes(
        type_: ResponseType,
        class: ResponseClass,
        decoder: &mut Decoder,
    ) -> Result<Self, ()> {
        Err(())
    }
}

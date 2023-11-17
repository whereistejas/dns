use crate::{
    constants::{ResponseClass, ResponseType},
    decoder::Decoder,
    domain::Domain,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResponseRecord {
    name: Domain,
    type_: ResponseType,
    class: ResponseClass,
    ttl: u32,
    rd_length: u16,
    rdata: RData,
}

impl ResponseRecord {
    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Result<Self, ()> {
        let name = Domain::from_bytes(decoder)?;
        let type_ = decoder.read_u16().try_into().unwrap();
        let class = decoder.read_u16().try_into().unwrap();
        Ok(Self {
            name,
            type_,
            class,
            ttl: decoder.read_u32(),
            rd_length: decoder.read_u16(),
            rdata: RData::from_bytes(type_, class, decoder)?,
        })
    }
}

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

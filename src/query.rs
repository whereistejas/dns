use arrayvec::ArrayVec;

use crate::{
    constants::{QueryClass, QueryType},
    decoder::Decoder,
    domain::decode_domain,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Query {
    /// Encoded domain name.
    pub(crate) qname: ArrayVec<u8, 255>,
    pub(crate) qtype: QueryType,
    pub(crate) qclass: QueryClass,
}

impl Query {
    pub(crate) fn as_bytes(&self) -> ArrayVec<u8, 259> {
        let mut bytes = ArrayVec::<_, 259>::new();
        bytes.try_extend_from_slice(self.qname.as_slice()).unwrap();
        bytes
            .try_extend_from_slice(u16::to_be_bytes(self.qtype as u16).as_slice())
            .unwrap();
        bytes
            .try_extend_from_slice(u16::to_be_bytes(self.qclass as u16).as_slice())
            .unwrap();

        bytes
    }

    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Result<Self, ()> {
        Ok(Self {
            qname: decode_domain(decoder)?,
            qtype: decoder.read_u16().try_into().unwrap(),
            qclass: decoder.read_u16().try_into().unwrap(),
        })
    }
}

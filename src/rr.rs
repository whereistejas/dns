use crate::{
    constants::{ResponseClass, ResponseType},
    decoder::Decoder,
    domain::Domain,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseRecord {
    name: Domain,
    type_: ResponseType,
    class: ResponseClass,
    ttl: u32,
    rd_length: u16,
    pub r_data: RData,
}

impl ResponseRecord {
    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Self {
        let name = Domain::from_bytes(decoder);
        let type_ = decoder.read_u16().try_into().unwrap();
        let class = decoder.read_u16().try_into().unwrap();

        Self {
            name,
            type_,
            class,
            ttl: decoder.read_u32(),
            rd_length: decoder.read_u16(),
            r_data: RData::from_bytes(type_, class, decoder),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RData {
    /// the canonical name for an alias
    CNAME(Domain),
    /// a host address
    A([u8; 4]),
    /// an authoritative name server
    NS(Domain),
}

impl RData {
    fn from_bytes(type_: ResponseType, class: ResponseClass, decoder: &mut Decoder) -> Self {
        match (class, type_) {
            (ResponseClass::IN, ResponseType::CNAME) => {
                let domain = Domain::from_bytes(decoder);
                Self::CNAME(domain)
            }
            (ResponseClass::IN, ResponseType::A) => {
                let ip_addr = [
                    decoder.pop().unwrap(),
                    decoder.pop().unwrap(),
                    decoder.pop().unwrap(),
                    decoder.pop().unwrap(),
                ];
                Self::A(ip_addr)
            }
            (ResponseClass::IN, ResponseType::NS) => {
                let domain = Domain::from_bytes(decoder);
                Self::NS(domain)
            }
            (_, _) => unimplemented!(),
        }
    }
}

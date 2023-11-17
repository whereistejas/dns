use crate::{decoder::Decoder, header::Header, query::Query, rr::ResponseRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    header: Header,
    question: Query,
    answer: Vec<ResponseRecord>,
    authority: Vec<ResponseRecord>,
    additional: Vec<ResponseRecord>,
}

impl Message {
    pub(crate) fn from_bytes(_decoder: Decoder) -> Result<Self, ()> {
        Err(())
    }
}

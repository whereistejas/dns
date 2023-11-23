use crate::{decoder::Decoder, header::Header, query::Query, rr::ResponseRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    header: Header,
    question: Query,
    // TODO: Replace all `Vec`s with `TinyVec`s.
    pub answer: Vec<ResponseRecord>,
    pub authority: Vec<ResponseRecord>,
    pub additional: Vec<ResponseRecord>,
}

// TODO: Create a global Result type in lib.rs
impl Message {
    pub(crate) fn decode(mut decoder: Decoder) -> Self {
        let header = Header::decode(&mut decoder);

        Self {
            header,
            question: Query::decode(&mut decoder),
            answer: (0..header.an_count)
                .map(|_| ResponseRecord::decode(&mut decoder))
                .collect::<Vec<_>>(),
            authority: (0..header.ns_count)
                .map(|_| ResponseRecord::decode(&mut decoder))
                .collect::<Vec<_>>(),
            additional: (0..header.ad_count)
                .map(|_| ResponseRecord::decode(&mut decoder))
                .collect::<Vec<_>>(),
        }
    }
}

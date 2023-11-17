use crate::{header::Header, query::Query, rr::ResponseRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Message {
    pub(crate) header: Header,
    pub(crate) question: Query,
    pub(crate) answer: Vec<ResponseRecord>,
    pub(crate) authority: Vec<ResponseRecord>,
    pub(crate) additional: Vec<ResponseRecord>,
}

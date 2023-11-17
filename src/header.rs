#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) id: u16,
    pub(crate) flags: u16,
    pub(crate) qd_count: u16,
    pub(crate) an_count: u16,
    pub(crate) ns_count: u16,
    pub(crate) ad_count: u16,
}

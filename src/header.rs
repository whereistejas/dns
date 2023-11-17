#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) id: u16,
    pub(crate) flags: u16,
    pub(crate) qd_count: u16,
    pub(crate) an_count: u16,
    pub(crate) ns_count: u16,
    pub(crate) ad_count: u16,
}

impl Header {
    pub(crate) fn new(id: u16, flags: u16) -> Self {
        Self {
            id,
            flags,
            qd_count: 0,
            an_count: 0,
            ns_count: 0,
            ad_count: 0,
        }
    }
    pub(crate) fn as_bytes(&self) -> [u8; 12] {
        [
            self.id.to_be_bytes()[0],
            self.id.to_be_bytes()[1],
            self.flags.to_be_bytes()[0],
            self.flags.to_be_bytes()[1],
            self.qd_count.to_be_bytes()[0],
            self.qd_count.to_be_bytes()[1],
            self.an_count.to_be_bytes()[0],
            self.an_count.to_be_bytes()[1],
            self.ns_count.to_be_bytes()[0],
            self.ns_count.to_be_bytes()[1],
            self.ad_count.to_be_bytes()[0],
            self.ad_count.to_be_bytes()[1],
        ]
    }
}
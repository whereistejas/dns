use crate::{decoder::Decoder, header::Header, query::Query, rr::ResponseRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    header: Header,
    question: Query,
    answer: Vec<ResponseRecord>,
    authority: Vec<ResponseRecord>,
    additional: Vec<ResponseRecord>,
}

// TODO: Create a global Result type in lib.rs
type ResultVec = Result<Vec<ResponseRecord>, ()>;

impl Message {
    pub(crate) fn from_bytes(mut decoder: Decoder) -> Result<Self, ()> {
        let header = Header::from_bytes(&mut decoder)?;
        let question = Query::from_bytes(&mut decoder)?;

        let answer = (0..header.an_count)
            .map(|_| ResponseRecord::from_bytes(&mut decoder))
            .collect::<ResultVec>()?;

        let authority = (0..header.ns_count)
            .map(|_| ResponseRecord::from_bytes(&mut decoder))
            .collect::<ResultVec>()?;

        let additional = (0..header.ad_count)
            .map(|_| ResponseRecord::from_bytes(&mut decoder))
            .collect::<ResultVec>()?;

        Ok(Self {
            header,
            question,
            answer,
            authority,
            additional,
        })
    }
}

#[test]
pub(crate) fn decode_response_packet() {
    #[rustfmt::skip]
    let buf: Vec<u8> = vec![
        0x10, 0x00, 0x81,
        0x80, // id = 4096, response, op=query, recursion_desired, recursion_available, no_error
        0x00, 0x01, 0x00, 0x01, // 1 query, 1 answer,
        0x00, 0x00, 0x00, 0x00, // 0 nameservers, 0 additional record
        0x03, b'w', b'w', b'w', // query --- www.example.com
        0x07, b'e', b'x', b'a', //
        b'm', b'p', b'l', b'e', //
        0x03, b'c', b'o', b'm', //
        0x00,                   // 0 = endname
        0x00, 0x01, 0x00, 0x01, // ReordType = A, Class = IN
        0xC0, 0x0C,             // name pointer to www.example.com
        0x00, 0x01, 0x00, 0x01, // RecordType = A, Class = IN
        0x00, 0x00, 0x00, 0x02, // TTL = 2 seconds
        0x00, 0x04,             // record length = 4 (ipv4 address)
        0x5D, 0xB8, 0xD8, 0x22, // address = 93.184.216.34
    ];

    let decoder = Decoder::new(buf.as_slice());

    assert!(Message::from_bytes(decoder).is_ok());
}

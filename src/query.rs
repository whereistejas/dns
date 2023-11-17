use arrayvec::ArrayVec;

use crate::{
    constants::{QueryClass, QueryType},
    header::Header,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Query {
    /// Encoded domain name.
    qname: ArrayVec<u8, 255>,
    qtype: QueryType,
    qclass: QueryClass,
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
}

fn encode_domain(domain: &str) -> ArrayVec<u8, 255> {
    domain
        .split('.')
        .flat_map(|part| {
            let mut label = ArrayVec::<_, 63>::new();
            label.push(u8::try_from(part.len()).unwrap());
            label.try_extend_from_slice(part.as_bytes()).unwrap();
            label
        })
        .chain([0])
        .collect()
}

#[test]
fn test_domain_encoding() {
    assert_eq!(
        &[7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0],
        encode_domain("example.com").as_slice()
    );
}

pub fn build_query(id: u16, type_: QueryType, domain: &str) -> ArrayVec<u8, 271> {
    assert!(
        domain.as_bytes().len() <= 255,
        "Domain name cannot have more than 255 octets"
    );

    let mut query = ArrayVec::<u8, 271>::new();

    let mut header = Header::new(id, 1 << 8);
    header.qd_count += 1;
    let question = Query {
        qname: encode_domain(domain),
        qtype: type_,
        qclass: QueryClass::IN,
    };

    query
        .try_extend_from_slice(header.as_bytes().as_slice())
        .unwrap();
    query
        .try_extend_from_slice(question.as_bytes().as_slice())
        .unwrap();

    query
}

#[test]
fn check_query_hex() {
    let query = build_query(0x3c5f, QueryType::A, "www.example.com");
    assert_eq!(
        "3c5f0100000100000000000003777777076578616d706c6503636f6d0000010001",
        hex::encode(query),
    )
}

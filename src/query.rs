use arrayvec::ArrayVec;

use crate::{
    constants::{QueryClass, QueryType},
    decoder::Decoder,
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

    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Result<Self, ()> {
        Ok(Self {
            qname: decode_domain(decoder)?,
            qtype: decoder.read_u16().try_into().unwrap(),
            qclass: decoder.read_u16().try_into().unwrap(),
        })
    }
}

fn decode_domain(decoder: &mut Decoder) -> Result<ArrayVec<u8, 255>, ()> {
    let mut domain = ArrayVec::<_, 255>::new();

    loop {
        let label_or_pointer = decoder.peek().unwrap();

        match label_or_pointer {
            0 => break,
            octet if octet >> 6 == 0 => {
                domain.try_extend_from_slice(&read_label(decoder)).unwrap();
            }
            octet if octet >> 6 == 3 => {
                let pointer = usize::try_from(octet & 0b00111111).unwrap();
                let mut decoder = decoder.clone_at_index(pointer);
                domain
                    .try_extend_from_slice(&read_label(&mut decoder))
                    .unwrap()
            }
            _ => return Err(()),
        }
    }

    Ok(domain)
}
fn read_label(decoder: &mut Decoder) -> ArrayVec<u8, 63> {
    let length = usize::try_from(decoder.pop().unwrap()).unwrap();
    let label = decoder.read_slice(length);
    ArrayVec::<_, 63>::try_from(label).unwrap()
}

// TODO: Add a test for pointers.
#[test]
fn check_domain_decoding() {
    let domain = &[
        7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
    ];
    let mut decoder = Decoder::new(domain);
    assert!(decode_domain(&mut decoder).is_ok());
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
fn check_domain_encoding() {
    assert_eq!(
        &[7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0],
        encode_domain("example.com").as_slice()
    );
}

use std::net::{IpAddr, SocketAddr, UdpSocket};

use arrayvec::ArrayVec;

use crate::{
    constants::{QueryClass, QueryType, ResponseClass, ResponseType},
    decoder::Decoder,
};

mod constants;
mod decoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Header {
    id: u16,
    flags: u16,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ad_count: u16,
}

impl Header {
    fn new(id: u16, flags: u16) -> Self {
        Self {
            id,
            flags,
            qd_count: 0,
            an_count: 0,
            ns_count: 0,
            ad_count: 0,
        }
    }
    fn encode(&self) -> [u8; 12] {
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
    fn decode(decoder: &mut Decoder) -> Self {
        Self {
            id: decoder.read_u16(),
            flags: decoder.read_u16(),
            qd_count: decoder.read_u16(),
            an_count: decoder.read_u16(),
            ns_count: decoder.read_u16(),
            ad_count: decoder.read_u16(),
        }
    }
}

/// Encoded domain name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain(ArrayVec<u8, 255>);

impl Domain {
    fn from_iter(labels: impl Iterator<Item = Label>) -> Self {
        let mut me = ArrayVec::new();

        for label in labels {
            match label {
                Label::Part(part) => me.extend(part.into_iter()),
                Label::Empty => me.extend([0]),
            }
        }

        Self(me)
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut decoder = Decoder::new(self.as_bytes());

        decode_domain(&mut decoder)
            .map(|label| label.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(".")
    }
}

fn encode_domain(domain: &str) -> impl Iterator<Item = Label> + '_ {
    domain.split('.').map(Label::encode).chain([Label::Empty])
}
fn decode_domain(decoder: &mut Decoder) -> impl Iterator<Item = Label> {
    let mut labels = vec![];

    loop {
        match decoder.peek().unwrap() {
            0 => {
                decoder.pop().unwrap();
                labels.push(Label::Empty);

                return labels.into_iter();
            }
            octet if octet >> 6 == 0 => {
                let value = Label::decode(decoder);
                labels.push(value);
            }
            octet if octet >> 6 == 3 => {
                let pointer = u16::from_be_bytes([
                    decoder.pop().unwrap() & 0b00111111,
                    decoder.pop().unwrap(),
                ])
                .into();

                return decode_domain(&mut decoder.clone_at_index(pointer));
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    Part(ArrayVec<u8, 63>),
    Empty,
}
impl Label {
    fn encode(part: &str) -> Self {
        let mut label = ArrayVec::new();

        label.push(u8::try_from(part.len()).unwrap());
        label.try_extend_from_slice(part.as_bytes()).unwrap();

        Self::Part(label)
    }

    fn decode(decoder: &mut Decoder) -> Self {
        let mut label = ArrayVec::new();

        let length = usize::from(decoder.pop().unwrap());
        label.push(u8::try_from(length).unwrap());
        label
            .try_extend_from_slice(decoder.read_slice(length))
            .unwrap();

        Self::Part(label)
    }

    #[allow(dead_code)]
    fn as_str(&self) -> &str {
        match self {
            Label::Part(part) => std::str::from_utf8(part).unwrap(),
            Label::Empty => "",
        }
    }
}

#[test]
fn domain() {
    assert_eq!(
        Domain::from_iter(encode_domain("example.com")),
        Domain::from_iter(decode_domain(&mut Decoder::new(&[
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ])))
    )
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Query {
    qname: Domain,
    qtype: QueryType,
    qclass: QueryClass,
}

impl Query {
    fn encode(&self) -> ArrayVec<u8, 259> {
        let mut bytes = ArrayVec::<_, 259>::new();
        bytes.try_extend_from_slice(self.qname.as_bytes()).unwrap();
        bytes
            .try_extend_from_slice(u16::to_be_bytes(self.qtype as u16).as_slice())
            .unwrap();
        bytes
            .try_extend_from_slice(u16::to_be_bytes(self.qclass as u16).as_slice())
            .unwrap();

        bytes
    }

    fn decode(decoder: &mut Decoder) -> Self {
        Self {
            qname: Domain::from_iter(decode_domain(decoder)),
            qtype: decoder.read_u16().try_into().unwrap(),
            qclass: decoder.read_u16().try_into().unwrap(),
        }
    }
}

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
    fn decode(decoder: &mut Decoder) -> Self {
        let name = Domain::from_iter(decode_domain(decoder));
        let type_ = decoder.read_u16().try_into().unwrap();
        let class = decoder.read_u16().try_into().unwrap();

        let ttl = decoder.read_u32();
        let rd_length = decoder.read_u16();
        // TODO: Use `rd_length` to assert that we have read exactly that many bytes.
        let r_data = RData::decode(type_, class, decoder);

        Self {
            name,
            type_,
            class,
            ttl,
            rd_length,
            r_data,
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
    fn decode(type_: ResponseType, class: ResponseClass, decoder: &mut Decoder) -> Self {
        match (class, type_) {
            (ResponseClass::IN, ResponseType::CNAME) => {
                let domain = Domain::from_iter(decode_domain(decoder));
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
                let domain = Domain::from_iter(decode_domain(decoder));
                Self::NS(domain)
            }
            (_, _) => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    header: Header,
    question: Query,
    pub answer: Vec<ResponseRecord>,
    pub authority: Vec<ResponseRecord>,
    pub additional: Vec<ResponseRecord>,
}

impl Message {
    fn decode(mut decoder: Decoder) -> Self {
        let header = Header::decode(&mut decoder);
        let question = Query::decode(&mut decoder);

        let answer = (0..header.an_count)
            .map(|_| ResponseRecord::decode(&mut decoder))
            .collect::<Vec<_>>();
        let authority = (0..header.ns_count)
            .map(|_| ResponseRecord::decode(&mut decoder))
            .collect::<Vec<_>>();
        let additional = (0..header.ad_count)
            .map(|_| ResponseRecord::decode(&mut decoder))
            .collect::<Vec<_>>();

        assert_eq!(decoder.len(), decoder.current());

        Self {
            header,
            question,
            answer,
            authority,
            additional,
        }
    }
}
pub fn build_query(id: u16, type_: QueryType, domain: &str) -> ArrayVec<u8, 281> {
    assert!(
        domain.as_bytes().len() <= 255,
        "Domain name cannot have more than 255 octets"
    );
    let mut query = ArrayVec::new();

    let mut header = Header::new(id, 1 << 8);
    header.qd_count += 1;
    query.extend(header.encode());

    let question = Query {
        qname: Domain::from_iter(encode_domain(domain)),
        qtype: type_,
        qclass: QueryClass::IN,
    };
    query.extend(question.encode());

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

pub fn send_query(domain: &str, name_server: IpAddr) -> Message {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to a random UDP address.");
    let query = build_query(rand::random(), QueryType::A, domain);

    socket
        .send_to(&query, SocketAddr::new(name_server, 53))
        .expect("Sending DNS query to name server.");

    let mut buf = [0; 512];
    let (bytes_recv, _) = socket
        .recv_from(&mut buf)
        .expect("Received a valid response from name server.");
    let mut buffer = ArrayVec::<_, 512>::new();
    buffer.try_extend_from_slice(&buf[..bytes_recv]).unwrap();

    Message::decode(Decoder::new(&buffer))
}

#[test]
fn example_com() {
    let response = send_query("www.example.com", "8.8.8.8".parse().unwrap());

    assert!(response
        .answer
        .iter()
        .any(|record| record.r_data == RData::A([93, 184, 216, 34,])));
}

use std::net::{IpAddr, SocketAddr, UdpSocket};

use arrayvec::ArrayVec;

use crate::{
    constants::{QueryClass, QueryType},
    decoder::Decoder,
    domain::Domain,
    header::Header,
    message::Message,
    query::Query,
};

mod constants;
mod domain;
mod header;
mod message;
mod query;
mod rr;

mod decoder;

pub fn build_query(id: u16, type_: QueryType, domain: &str) -> ArrayVec<u8, 271> {
    assert!(
        domain.as_bytes().len() <= 255,
        "Domain name cannot have more than 255 octets"
    );

    let mut query = ArrayVec::<u8, 271>::new();

    let mut header = Header::new(id, 1 << 8);
    header.qd_count += 1;
    let question = Query {
        qname: Domain::new(domain),
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

pub fn send_query(domain: &str, name_server: IpAddr) -> Result<Message, ()> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to a random UDP address.");
    let query = build_query(rand::random(), QueryType::A, domain);

    socket
        .send_to(&query, SocketAddr::new(name_server, 53))
        .expect("Sending DNS query to name server.");

    let mut buf = [0; 512];
    let (bytes_recv, _) = socket
        .recv_from(&mut buf)
        .expect("Received a valid response from name server.");

    assert!(bytes_recv < 512);

    let decoder = Decoder::new(buf.as_slice());

    Message::from_bytes(decoder)
}

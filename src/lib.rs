#![feature(exact_size_is_empty)]

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
        qname: Domain::from_iter(domain::encode(domain)),
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
    assert!(bytes_recv < 512);

    let mut buffer = ArrayVec::<_, 512>::new();
    buffer.extend(buf);

    Message::decode(Decoder::new(&buffer))
}

#[test]
fn example_com() {
    use rr::RData;

    let response = send_query("www.example.com", "8.8.8.8".parse().unwrap());

    assert!(response
        .answer
        .iter()
        .any(|record| record.r_data == RData::A([93, 184, 216, 34,])));
}

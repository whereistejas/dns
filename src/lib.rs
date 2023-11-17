use std::net::{IpAddr, SocketAddr, UdpSocket};

use crate::constants::QueryType;

mod constants;
mod header;
mod message;
mod query;
mod rr;

pub fn send_query(domain: &str, name_server: IpAddr) -> [u8; 512] {
mod decoder;
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to a random UDP address.");
    let query = query::build_query(rand::random(), QueryType::A, domain);

    socket
        .send_to(&query, SocketAddr::new(name_server, 53))
        .expect("Sending DNS query to name server.");

    let mut buf = [0; 512];
    let (bytes_recv, _) = socket
        .recv_from(&mut buf)
        .expect("Received a valid response from name server.");
    assert!(bytes_recv < 512);
    buf
}

#[ignore]
#[test]
fn example_com() {
    let resp = send_query("www.example.com", "8.8.8.8".parse().unwrap());
    assert!(!resp.is_empty());
    panic!("{resp:?}");
}

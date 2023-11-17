use std::net::{IpAddr, SocketAddr, UdpSocket};

use crate::{constants::QueryType, decoder::Decoder, message::Message};

mod constants;
mod header;
mod message;
mod query;
mod rr;

mod decoder;

pub fn send_query(domain: &str, name_server: IpAddr) -> Result<Message, ()> {
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

    let decoder = Decoder::new(buf.as_slice());

    Message::from_bytes(decoder)
}


use arrayvec::ArrayVec;

use crate::decoder::Decoder;

pub(crate) fn decode_domain(decoder: &mut Decoder) -> Result<ArrayVec<u8, 255>, ()> {
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

pub(crate) fn encode_domain(domain: &str) -> ArrayVec<u8, 255> {
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

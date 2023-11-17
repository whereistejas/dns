use arrayvec::ArrayVec;

use crate::decoder::Decoder;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Encoded domain.
pub struct Domain(ArrayVec<u8, 255>);

impl Domain {
    pub(crate) fn new(domain: &str) -> Self {
        Self(
            domain
                .split('.')
                .flat_map(|part| {
                    let mut label = ArrayVec::<_, 63>::new();
                    label.push(u8::try_from(part.len()).unwrap());
                    label.try_extend_from_slice(part.as_bytes()).unwrap();
                    label
                })
                .chain([0])
                .collect(),
        )
    }
    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Result<Self, ()> {
        let mut domain = ArrayVec::<_, 255>::new();

        loop {
            let label_or_pointer = decoder.peek().unwrap();

            match label_or_pointer {
                0 => break,
                octet if octet >> 6 == 0 => {
                    domain
                        .try_extend_from_slice(&Self::read_label(decoder))
                        .unwrap();
                }
                octet if octet >> 6 == 3 => {
                    let pointer = usize::try_from(octet & 0b00111111).unwrap();
                    let mut decoder = decoder.clone_at_index(pointer);
                    domain
                        .try_extend_from_slice(&Self::read_label(&mut decoder))
                        .unwrap()
                }
                _ => return Err(()),
            }
        }

        Ok(Self(domain))
    }
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
    fn read_label<'a>(decoder: &'a mut Decoder) -> &'a [u8] {
        let length = usize::try_from(decoder.pop().unwrap()).unwrap();
        assert!(length <= 63);
        decoder.read_slice(length)
    }
}

// TODO: Add a test for pointers.
// #[test]
// fn check_domain_decoding() {
//     let domain = &[
//         7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
//     ];
//     let mut decoder = Decoder::new(domain);
//     assert!(decode_domain(&mut decoder).is_ok());
// }

// #[test]
// fn check_domain_encoding() {
//     assert_eq!(
//         Domain(
//             ArrayVec::<_, 255>::try_from(
//                 [7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0].as_slice()
//             )
//             .unwrap()
//         ),
//         encode_domain("example.com")
//     );
// }

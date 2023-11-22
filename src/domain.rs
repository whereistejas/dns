use arrayvec::ArrayVec;

use crate::decoder::Decoder;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Encoded domain name.
pub struct Domain(ArrayVec<u8, 255>);

impl Domain {
    pub(crate) fn new(domain: &str) -> Self {
        Self(ArrayVec::<_, 255>::from_iter(
            domain
                .split('.')
                .flat_map(|part| {
                    let label = Label::new(part);
                    label.as_bytes()
                })
                .chain([0]),
        ))
    }

    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Result<Self, ()> {
        let mut domain = Domain::new("");

        loop {
            match decoder.peek() {
                Some(0) => break,
                Some(octet) if octet >> 6 == 0 => {
                    domain.push_label(Label::from_bytes(decoder));
                }
                Some(octet) if octet >> 6 == 3 => {
                    let pointer = usize::try_from(octet & 0b00111111).unwrap();
                    let mut decoder = decoder.clone_at_index(pointer);
                    domain.push_label(Label::from_bytes(&mut decoder));
                }
                _ => return Err(()),
            }
        }

        Ok(domain)
    }
    fn push_label(&mut self, label: Label) {
        self.0.push(label.len().try_into().unwrap());
        self.0.try_extend_from_slice(&label.as_bytes()).unwrap()
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[allow(dead_code)]
    pub(crate) fn as_str<'a>(&'a self) -> &'a str {
        std::str::from_utf8(self.0.as_slice()).unwrap()
    }
}

pub struct Label(ArrayVec<u8, 63>);
impl Label {
    fn new(part: &str) -> Self {
        let mut label = ArrayVec::<_, 63>::new();
        label.push(u8::try_from(part.len()).unwrap());
        label.try_extend_from_slice(part.as_bytes()).unwrap();
        Self(label)
    }
    fn from_bytes(decoder: &mut Decoder) -> Self {
        let mut label = ArrayVec::<_, 63>::new();
        let length = usize::try_from(decoder.pop().unwrap()).unwrap();
        assert!(length <= 63);
        label
            .try_extend_from_slice(decoder.read_slice(length))
            .unwrap();

        Self(label)
    }
    fn as_bytes(self) -> ArrayVec<u8, 63> {
        self.0
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

// TODO: Add a test for pointers.
#[test]
fn decode_domain() {
    let domain = [
        7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
    ];
    assert!(Domain::from_bytes(&mut Decoder::new(&domain)).is_ok())
}

#[test]
fn encode_domain() {
    assert_eq!(
        [7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0].as_slice(),
        Domain::new("example.com").as_bytes()
    );
}

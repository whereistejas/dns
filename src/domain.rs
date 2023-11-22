use std::str;

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
                .flat_map(|part| Label::new(part).as_bytes())
                .chain([0]),
        ))
    }

    fn labels(decoder: &mut Decoder) -> impl ExactSizeIterator<Item = Label> {
        let mut labels = vec![];

        loop {
            match decoder.peek() {
                Some(0) => {
                    // Pop terminating octet.
                    decoder.pop().unwrap();
                    return labels.into_iter();
                }
                Some(octet) if octet >> 6 == 0 => {
                    let value = Label::from_bytes(decoder);
                    labels.push(value);
                }
                Some(octet) if octet >> 6 == 3 => {
                    let pointer = u16::from_be_bytes([
                        decoder.pop().unwrap() & 0b00111111,
                        decoder.pop().unwrap(),
                    ]);
                    let pointer = usize::try_from(pointer).unwrap();
                    return Self::labels(&mut decoder.clone_at_index(pointer));
                }
                octet => panic!("\n{decoder:?}\nweird octet: {octet:?}"),
            }
        }
    }

    pub(crate) fn from_bytes(decoder: &mut Decoder) -> Self {
        // An empty domain name is a valid domain name.
        let mut domain = Domain(ArrayVec::<_, 255>::new());

        for label in Self::labels(decoder) {
            domain.push_label(label);
        }
        domain.add_terminator();

        domain
    }
    fn push_label(&mut self, label: Label) {
        self.0.push(label.len().try_into().unwrap());
        self.0.try_extend_from_slice(&label.as_bytes()).unwrap();
    }
    fn add_terminator(&mut self) {
        self.0.push(0);
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[allow(dead_code)]
    pub(crate) fn display<'a>(&'a self) -> String {
        let mut decoder = Decoder::new(self.as_bytes());

        Self::labels(&mut decoder)
            .map(|label| label.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    #[allow(dead_code)]
    fn as_str<'a>(&'a self) -> &'a str {
        std::str::from_utf8(self.0.as_slice()).unwrap()
    }
}

// TODO: Add a test for pointers.
#[test]
fn domain() {
    let domain = [
        7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
    ];
    assert_eq!(
        Domain::from_bytes(&mut Decoder::new(&domain)),
        Domain::new("example.com")
    )
}

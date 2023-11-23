use arrayvec::ArrayVec;

use crate::decoder::Decoder;

/// Encoded domain name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain(ArrayVec<u8, 255>);

impl Domain {
    pub(crate) fn from_iter(labels: impl Iterator<Item = Label>) -> Self {
        let mut me = ArrayVec::new();

        for label in labels {
            match label {
                Label::Part(part) => me.extend(part.into_iter()),
                Label::Empty => me.extend([0]),
            }
        }

        Self(me)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[allow(dead_code)]
    pub(crate) fn display<'a>(&'a self) -> String {
        let mut decoder = Decoder::new(self.as_bytes());

        decode(&mut decoder)
            .map(|label| label.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(".")
    }
}

pub(crate) fn encode<'a>(domain: &'a str) -> impl Iterator<Item = Label> + 'a {
    domain
        .split('.')
        .into_iter()
        .map(|part| Label::encode(part))
        .chain([Label::Empty])
}
pub(crate) fn decode(decoder: &mut Decoder) -> impl Iterator<Item = Label> {
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
                .try_into()
                .unwrap();

                return decode(&mut decoder.clone_at_index(pointer));
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

        let length = usize::try_from(decoder.pop().unwrap()).unwrap();
        label.push(u8::try_from(length).unwrap());
        label
            .try_extend_from_slice(decoder.read_slice(length))
            .unwrap();

        Self::Part(label)
    }

    #[allow(dead_code)]
    fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Label::Part(part) => std::str::from_utf8(&part).unwrap(),
            Label::Empty => "",
        }
    }
}

// TODO: Add a test for pointers.
#[test]
fn domain() {
    assert_eq!(
        Domain::from_iter(encode("example.com")),
        Domain::from_iter(decode(&mut Decoder::new(&[
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ])))
    )
}

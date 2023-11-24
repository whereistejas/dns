use arrayvec::ArrayVec;

pub struct Encoder {
    buffer: ArrayVec<u8, 512>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            buffer: ArrayVec::new(),
        }
    }

    pub fn try_write_u16(&mut self, value: u16) -> Result<(), ()> {
        self.try_write_slice(value.to_be_bytes().as_slice())
    }
    pub fn try_write_slice<'a>(&'a mut self, value: &'a [u8]) -> Result<(), ()> {
        self.buffer.try_extend_from_slice(value).map_err(|_| ())
    }

    pub fn bytes(self) -> ArrayVec<u8, 512> {
        self.buffer
    }
}

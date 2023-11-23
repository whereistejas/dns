#![allow(dead_code)]

use std::fmt;

/// This idea for a decoder is borrowed from Hickory DNS's BinEncoder.
pub struct Decoder<'a> {
    /// the original, is never modified
    buffer: &'a [u8],
    remaining: &'a [u8],
}

impl<'a> fmt::Debug for Decoder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Buffer: {:?}\nRemaining: {:?}\nCurrent: {}",
            self.buffer,
            self.remaining,
            self.current()
        ))
    }
}

impl<'a> Decoder<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            remaining: buffer,
        }
    }
    pub fn current(&self) -> usize {
        self.buffer.len() - self.remaining.len()
    }
    /// Returns the next byte without incrementing the index.
    /// If we are at the end of the buffer returns [None].
    pub fn peek(&self) -> Option<u8> {
        (!self.remaining.is_empty()).then(|| self.remaining[0])
    }
    /// Returns the next byte incrementing the index.
    /// If we are at the end of the buffer returns [None].
    pub fn pop(&mut self) -> Option<u8> {
        if let Some((&first, remaining)) = self.remaining.split_first() {
            self.remaining = remaining;
            Some(first)
        } else {
            None
        }
    }
    pub fn read_slice(&mut self, offset: usize) -> &'a [u8] {
        let (result, remaining) = self.remaining.split_at(offset);
        self.remaining = remaining;
        result
    }
    pub fn read_u16(&mut self) -> u16 {
        let s = self.read_slice(2);
        u16::from_be_bytes([s[0], s[1]])
    }
    pub fn read_u32(&mut self) -> u32 {
        let s = self.read_slice(4);
        u32::from_be_bytes([s[0], s[1], s[2], s[3]])
    }
    pub fn clone_at_index(&self, index: usize) -> Self {
        assert!(index < self.buffer.len());

        Self {
            buffer: self.buffer,
            remaining: &self.buffer[index..],
        }
    }
}

use crate::byte_utils::parse_string;
#[cfg(feature = "serde")]
use serde::Serialize;
use std::string::String as StdString;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct String {
    value: StdString,
}

impl String {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        offset += 1; // padding

        offset += 1; // String count, should always be 4

        let (bytes_read, value): (usize, StdString) = parse_string(&bytes[offset..]);
        offset += bytes_read;

        (offset, Self {
            value
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut StdString {
        &mut self.value
    }
}
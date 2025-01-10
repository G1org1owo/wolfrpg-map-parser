use crate::byte_utils::{as_u32_le, parse_string};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct CSV {
    entry_count: u32,
    filename: String
}

impl CSV {
    pub(crate) fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let entry_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 1; // padding

        offset += 1; // String count, should always be 4

        let (bytes_read, filename): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;

        (offset, Self {
            entry_count,
            filename
        })
    }

    pub fn entry_count(&self) -> u32 {
        self.entry_count
    }

    pub fn entry_count_mut(&mut self) -> &mut u32 {
        &mut self.entry_count
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn filename_mut(&mut self) -> &mut String {
        &mut self.filename
    }
}
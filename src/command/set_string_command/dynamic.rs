use crate::byte_utils::as_u32_le;
use serde::Serialize;

#[derive(Serialize)]
pub struct Dynamic {
    source: u32
}

impl Dynamic {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let source: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            source
        })
    }
}
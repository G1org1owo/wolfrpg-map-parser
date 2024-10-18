use crate::byte_utils::as_u32_le;
use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    unknown1: u32,
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset = 0;

        let unknown1: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 1; // command end signature

        (offset, Self {
            unknown1,
        })
    }
}
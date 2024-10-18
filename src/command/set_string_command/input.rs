use crate::byte_utils::as_u32_le;
use serde::Serialize;

#[derive(Serialize)]
pub struct Input {
    input_max_length: u32
}

impl Input {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let input_max_length: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            input_max_length
        })
    }
}
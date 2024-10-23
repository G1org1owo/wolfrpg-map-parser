use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct Keyboard {
    options: [u8; 3],
    key_code: u32
}

impl Keyboard {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: [u8; 3] = bytes[offset..offset + 3].try_into().unwrap();
        offset += 3;

        offset += 1; // input_type

        let key_code: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        (offset, Self {
            options,
            key_code
        })
    }
}
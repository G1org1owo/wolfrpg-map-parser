use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::input_key_command::input_key::key_options::KeyOptions;

#[derive(Serialize)]
pub struct KeyboardOrPad {
    options: KeyOptions,
    unknown1: u16,
    key_code: u32
}

impl KeyboardOrPad {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u8 = bytes[offset];
        let options: KeyOptions = KeyOptions::new(options);
        offset += 1;

        offset += 1; // input_type

        let unknown1: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        let key_code: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        (offset, Self {
            options,
            unknown1,
            key_code
        })
    }
}
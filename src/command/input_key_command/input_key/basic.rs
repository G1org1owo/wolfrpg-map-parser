use crate::byte_utils::as_u16_le;
use crate::command::input_key_command::input_key::basic_options::BasicOptions;
use serde::Serialize;

#[derive(Serialize)]
pub struct Basic {
    options: BasicOptions,
    unknown1: u16
}

impl Basic {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u8 = bytes[offset];
        let options: BasicOptions = BasicOptions::new(options);
        offset += 1;

        offset += 1; // input_type

        let unknown1: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        (offset, Self {
            options,
            unknown1
        })
    }
}
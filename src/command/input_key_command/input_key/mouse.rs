use crate::byte_utils::as_u16_le;
use crate::command::input_key_command::input_key::mouse_options::MouseOptions;
use serde::Serialize;

#[derive(Serialize)]
pub struct Mouse {
    options: MouseOptions,
    unknown1: u16
}

impl Mouse {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u8 = bytes[offset];
        let options: MouseOptions = MouseOptions::new(options);
        offset += 1;

        offset += 1; // input_Type

        let unknown1: u16 = as_u16_le(&bytes[offset..offset+2]);
        offset += 2;

        (offset, Self {
            options,
            unknown1
        })
    }
}
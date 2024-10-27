use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct ChangeColor {
    red: u8,
    green: u8,
    blue: u8,
    flash: bool,
    duration: u32
}

impl ChangeColor {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let red: u8 = bytes[offset];
        offset += 1;

        let green: u8 = bytes[offset];
        offset += 1;

        let blue: u8 = bytes[offset];
        offset += 1;

        let flash: bool = bytes[offset] != 0;
        offset += 1;

        let duration: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            red,
            green,
            blue,
            flash,
            duration
        })
    }
}
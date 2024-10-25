use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::colors::Colors;

#[derive(Serialize)]
pub struct DelayState {
    colors: Colors,
    delay: u32
}

impl DelayState {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let colors: u8 = bytes[offset];
        let colors: Colors = Colors::new(colors);
        offset += 1;

        let delay: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            colors,
            delay
        })
    }
}
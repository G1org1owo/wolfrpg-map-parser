use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct Position {
    target: u8,
    unknown1: u8,
    position_x: u32,
    position_y: u32
}

impl Position {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let target: u8 = bytes[offset];
        let unknown1: u8 = bytes[offset+1];
        offset += 2;

        let position_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            target,
            unknown1,
            position_x,
            position_y,
        })
    }
}
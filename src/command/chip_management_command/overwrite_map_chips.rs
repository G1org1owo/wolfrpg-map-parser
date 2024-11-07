use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct OverwriteMapChips{
    layer: u32,
    position_x: u32,
    position_y: u32,
    width: u32,
    height: u32,
    chip: u32
}

impl OverwriteMapChips{
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let layer: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let width: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let height: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let chip: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            layer,
            position_x,
            position_y,
            width,
            height,
            chip
        })
    }
}
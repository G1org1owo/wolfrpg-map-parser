use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct SwitchChipset {
    chipset: u32
}

impl SwitchChipset {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let chipset: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            chipset
        })
    }
}
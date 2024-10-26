use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::erase::delay::Delay;

#[derive(Serialize)]
pub struct Range {
    delay_fields: Delay,
    unknown1: u32,
    range_count: u32
}

impl Range {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, delay_fields): (usize, Delay)
            = Delay::parse(&bytes[offset..]);
        offset += bytes_read;

        let unknown1: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let range_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            delay_fields,
            unknown1,
            range_count
        })
    }
}
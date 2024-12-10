use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct EraseEvent {
    event: u32,
    fade_frames: u32
}

impl EraseEvent {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let event: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let fade_frames: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, EraseEvent {
            event,
            fade_frames
        })
    }
}
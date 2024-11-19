use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};

#[derive(Serialize)]
pub struct SetTransition {
    transition_number: u32,
    fade_frames: u16,
    wait_until_done: bool
}

impl SetTransition {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let transition_number: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let fade_frames: u16 = as_u16_le(&bytes[offset..offset+2]);
        offset += 2;

        let wait_until_done: bool = bytes[offset] != 0;
        offset += 1;

        offset += 4; // Command end signature

        (offset, Self {
            transition_number,
            fade_frames,
            wait_until_done
        })
    }
}
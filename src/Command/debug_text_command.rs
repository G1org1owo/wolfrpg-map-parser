use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};

#[derive(Serialize)]
pub struct DebugTextCommand {
    text: String,
}

impl DebugTextCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset = 2;

        let text_length = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;

        let text: String = as_string(bytes, offset, text_length);
        offset += text_length;

        offset += 1; // Command end signature

        (offset, DebugTextCommand {
            text
        })
    }
}
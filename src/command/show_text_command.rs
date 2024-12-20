use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};

#[derive(Serialize)]
pub struct ShowTextCommand {
    text: String
}

impl ShowTextCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;
        offset += 2; // Unknown, probably padding

        let text_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;

        let text: String = as_string(bytes, offset, text_length);
        offset += text_length;

        offset += 1; // command end byte, should be 0x00

        (offset, Self {
            text
        })
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
}
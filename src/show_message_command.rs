use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct ShowMessageCommand {
    message: String
}

impl ShowMessageCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 3;

        let message_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;

        let message: String = String::from_utf8(bytes[offset..offset+message_length-1].to_vec())
            .unwrap();
        offset += message_length;
        offset += 1; // Command end byte, should be 0x00

        (offset, Self {
            message
        })
    }
}
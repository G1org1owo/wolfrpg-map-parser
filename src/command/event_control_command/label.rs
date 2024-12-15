use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};

#[derive(Serialize)]
pub struct Label {
    label: String
}

impl Label {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;
        offset += 2; // padding + string_count which are always 0x0001

        let label_length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
        offset += 4;

        let label: String = as_string(&bytes, offset, label_length);
        offset += label_length;

        offset += 1; // Command end

        (offset, Self {
            label
        })
    }
}
use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};

#[derive(Serialize)]
pub struct Base {
    unknown2: u8,
    string: String,
    replace: Option<String>
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let unknown2: u8 = bytes[offset];
        offset += 1;

        let string_count: u8 = bytes[offset];
        offset += 1;

        let string_length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
        offset += 4;
        let string: String = as_string(bytes, offset, string_length);
        offset += string_length;

        let replace: Option<String>;

        if string_count == 2 {
            let replace_length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
            offset += 4;
            let replace_string: String = as_string(bytes, offset, replace_length);
            offset += replace_length;

            replace = Some(replace_string);
        } else {
            replace = None;
        }

        offset += 1; // Command end signature

        (offset, Self {
            unknown2,
            string,
            replace
        })
    }
}
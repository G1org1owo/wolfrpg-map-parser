use serde::Serialize;
use crate::byte_utils::{as_string, as_u16_le, as_u32_le};
use crate::command::set_string_command::operation::Operation;
use crate::command::set_string_command::options::Options;

#[derive(Serialize)]
pub struct Base {
    variable: u32,
    options: Options,
    operation: Operation,
    unknown1: u16,
    unknown2: u8,
    string: String,
    replace: Option<String>
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let variable: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let options: u8 = bytes[offset];
        let options: Options = Options::new(options);

        let operation: u8 = bytes[offset+1];
        let operation: Operation = Operation::new(operation);
        offset += 2;

        let unknown1: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

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
            variable,
            options,
            operation,
            unknown1,
            unknown2,
            string,
            replace
        })
    }
}
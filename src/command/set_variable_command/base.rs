use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::set_variable_command::operators::Operators;
use crate::command::set_variable_command::options::Options;

#[derive(Serialize)]
pub struct Base {
    variable: u32,
    left_side: u32,
    right_side: u32,
    options: Options,
    operators: Operators,
    unknown1: u32,
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset = 0;

        let variable: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let left_side: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let right_side: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let options: u8 = bytes[offset];
        let options: Options = Options::new(options);

        let operators: u8 = bytes[offset + 1];
        let operators: Operators = Operators::new(operators);

        offset += 2;

        let unknown1: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 1; // command end signature

        (offset, Self {
            variable,
            left_side,
            right_side,
            options,
            operators,
            unknown1,
        })
    }
}
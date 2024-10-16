use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::set_variable_command::operators::Operators;
use crate::command::set_variable_command::options::Options;

#[derive(Serialize)]
pub struct Range {
    variable: u32,
    left_side: u32,
    right_side: u32,
    options: Options,
    operators: Operators,
    range_start: u16,
    range_end: u16,
    unknown1: u32,
}

impl Range {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset = 0;

        let variable = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let left_side = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let right_side = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let options: u8 = bytes[offset];
        let options: Options = Options::new(options);

        let operators: u8 = bytes[offset+1];
        let operators: Operators = Operators::new(operators);

        offset += 2;

        let range_start: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        let range_end: u16 = as_u16_le(&bytes[offset..offset + 2]);
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
            range_start,
            range_end,
            unknown1,
        })
    }
}
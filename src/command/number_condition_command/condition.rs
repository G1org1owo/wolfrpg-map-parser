use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::number_condition_command::operator::Operator;

#[derive(Serialize)]
pub struct Condition {
    variable: u32,
    value: u32,
    operator: Operator
}

impl Condition {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let variable: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let operator: Operator = Operator::new(bytes[offset]);
        offset += 1;

        offset += 3; // Padding

        (offset, Self {
            variable,
            value,
            operator
        })
    }
}
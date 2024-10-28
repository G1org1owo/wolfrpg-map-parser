use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::save_load_command::operation::Operation;

#[derive(Serialize)]
pub struct Base {
    operation: Operation,
    save_number: u32
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let operation: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let operation: Operation = Operation::new(operation);
        offset += 4;

        let save_number: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            operation,
            save_number
        })
    }
}
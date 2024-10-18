mod base;
mod options;
mod content_type;
mod variable_type;
mod string_operation;
mod operation;
mod set_string_command_state;

use serde::Serialize;
use set_string_command_state::SetStringCommandState;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::set_string_command::operation::Operation;
use crate::command::set_string_command::options::Options;

#[derive(Serialize)]
pub struct SetStringCommand {
    variable: u32,
    options: Options,
    operation: Operation,
    unknown1: u16,
    state: SetStringCommandState
}

impl SetStringCommand {
    fn parse(bytes: &[u8], parse_state: fn(&[u8]) -> (usize, SetStringCommandState)) -> (usize, Self) {
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

        let (bytes_read, state): (usize, SetStringCommandState) = parse_state(&bytes[offset..]);
        offset += bytes_read;

        (offset, Self {
            variable,
            options,
            operation,
            unknown1,
            state
        })
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, SetStringCommandState::parse_base)
    }
}
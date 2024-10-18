use serde::Serialize;
use state::State;
use crate::byte_utils::as_u32_le;
use crate::command::set_variable_command::operators::Operators;
use crate::command::set_variable_command::options::Options;
mod base;
mod assignment;
mod calculation;
mod options;
mod operators;
mod range;
mod state;

#[derive(Serialize)]
pub struct SetVariableCommand {
    variable: u32,
    left_side: u32,
    right_side: u32,
    options: Options,
    operators: Operators,
    state: State
}

impl SetVariableCommand {
    fn parse(bytes: &[u8], parse_state: fn(&[u8]) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

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

        let (bytes_read, state): (usize, State) = parse_state(bytes);

        offset += bytes_read;

        (offset, Self {
            variable,
            left_side,
            right_side,
            options,
            operators,
            state
        })
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_base)
    }

    pub fn parse_range(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_range)
    }
}
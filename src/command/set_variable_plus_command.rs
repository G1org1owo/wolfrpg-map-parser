use serde::Serialize;
use state::State;
use crate::byte_utils::as_u32_le;
use crate::command::set_variable_plus_command::assignment::Assignment;
use crate::command::set_variable_plus_command::options::Options;
use crate::command::set_variable_plus_command::variable_type::VariableType;

mod state;
mod character;
mod options;
mod variable_type;
mod assignment_operator;
mod assignment;
mod position;
mod picture;
mod picture_field;
mod other;
mod target;
mod character_field;

#[derive(Serialize)]
pub struct SetVariablePlusCommand {
    variable: u32,
    options: Options,
    assignment: Assignment,
    state: State
}

impl SetVariablePlusCommand {
    fn parse(bytes: &[u8], parse_state: fn(&[u8]) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

        let variable: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let options: u8 = bytes[offset];
        let options:Options = Options::new(options);

        let assignment: u8 = bytes[offset+1];
        let assignment: Assignment = Assignment::new(assignment);

        offset += 2;

        let (bytes_read, state): (usize, State) = parse_state(&bytes[offset..]);
        offset += bytes_read;

        (offset, Self {
            variable,
            options,
            assignment,
            state
        })
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        match Assignment::new(bytes[5]).variable_type() {
            VariableType::Character => Self::parse(bytes, State::parse_character),
            VariableType::Position => Self::parse(bytes, State::parse_position),
            VariableType::PictureNumber => Self::parse(bytes, State::parse_picture),
            _ => panic!("Invalid variable type: {:x}", bytes[5] & 0x0f)
        }
    }

    pub fn parse_other(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_other)
    }
}
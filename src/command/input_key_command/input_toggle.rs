use serde::Serialize;
use crate::command::input_key_command::input_toggle::input_type::InputType;
use crate::command::input_key_command::input_toggle::state::State;

mod input_type;
mod state;
mod basic;
mod basic_inputs;
mod enabled_inputs;
mod device;
mod device_inputs;

#[derive(Serialize)]
pub struct InputToggle {
    input_type: InputType,
    state: State
}

impl InputToggle {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let input_type: u8 = bytes[offset+3];
        let input_type: InputType = InputType::new(input_type);

        let (bytes_read, state): (usize, State) = State::parse(&bytes[offset..], &input_type);
        offset += bytes_read;

        offset += 3; // Command end signature

        (offset, Self {
            input_type,
            state
        })
    }

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn input_type_mut(&mut self) -> &mut InputType {
        &mut self.input_type
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}
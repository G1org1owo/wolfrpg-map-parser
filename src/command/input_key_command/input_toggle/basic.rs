use crate::command::input_key_command::input_toggle::basic_inputs::BasicInputs;
use crate::command::input_key_command::input_toggle::enabled_inputs::EnabledInputs;
use serde::Serialize;

#[derive(Serialize)]
pub struct Basic {
    inputs: BasicInputs,
    enabled_inputs: EnabledInputs,
    unknown1: u8,
}

impl Basic {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let inputs: u8 = bytes[offset];
        let inputs: BasicInputs = BasicInputs::new(inputs);
        offset += 1;

        let enabled_inputs: u8 = bytes[offset];
        let enabled_inputs: EnabledInputs = EnabledInputs::new(enabled_inputs);
        offset += 1;

        let unknown1: u8 = bytes[offset];
        offset += 1;

        offset += 1; // input_type

        (offset, Self {
            inputs,
            enabled_inputs,
            unknown1
        })
    }
}
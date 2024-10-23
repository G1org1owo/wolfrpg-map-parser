use serde::Serialize;
use crate::command::input_key_command::input_key::direction_keys::DirectionKeys;

#[derive(Serialize)]
pub struct BasicOptions {
    direction_keys: DirectionKeys,
    input_ok: bool,
    input_cancel: bool,
    input_subkey: bool,
    wait_for_input: bool
}

impl BasicOptions {
    pub fn new(options: u8) -> Self {
        Self {
            direction_keys: DirectionKeys::new(options & 0x0f),
            input_ok:       options & 0b00010000 != 0,
            input_cancel:   options & 0b00100000 != 0,
            input_subkey:   options & 0b01000000 != 0,
            wait_for_input: options & 0b10000000 != 0,
        }
    }
}
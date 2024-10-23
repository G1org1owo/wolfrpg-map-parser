use serde::Serialize;
use crate::command::input_key_command::input_key::mouse_target::MouseTarget;

#[derive(Serialize)]
pub struct MouseOptions {
    target: MouseTarget,
    left_click: bool,
    right_click: bool,
    middle_click: bool,
    wait_for_input: bool
}

impl MouseOptions {
    pub fn new(options: u8) -> Self {
        Self {
            target: MouseTarget::new(options & 0x0f),
            left_click:     options & 0b00010000 != 0,
            right_click:    options & 0b00100000 != 0,
            middle_click:   options & 0b01000000 != 0,
            wait_for_input: options & 0b10000000 != 0
        }
    }
}
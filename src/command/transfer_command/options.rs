use serde::Serialize;
use crate::command::transfer_command::transition::Transition;

#[derive(Serialize)]
pub struct Options {
    precise_coordinates: bool,
    transition: Transition,
}

impl Options {
    pub fn new(options: u32) -> Self {
        Self {
            precise_coordinates: options & 0b00000001 != 0,
            transition: Transition::new(((options >> 4) & 0x0f) as u8),
        }
    }
}
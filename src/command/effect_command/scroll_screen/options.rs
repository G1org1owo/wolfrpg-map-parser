use serde::Serialize;
use crate::command::effect_command::scroll_screen::scroll_operation::ScrollOperation;
use crate::command::effect_command::scroll_screen::scroll_speed::ScrollSpeed;

#[derive(Serialize)]
pub struct Options {
    scroll_operation: ScrollOperation,
    scroll_speed: ScrollSpeed,
    wait_until_done: bool,
    pixel_units: bool
}

impl Options {
    pub fn new(options: u32) -> Self {
        Self {
            scroll_operation: ScrollOperation::new((options & 0x0f) as u8),
            scroll_speed: ScrollSpeed::new(((options & 0xf0) >> 4) as u8),
            wait_until_done: (options >> 8) & 0b00000001 != 0,
            pixel_units:     (options >> 8) & 0b00000010 != 0,
        }
    }
}
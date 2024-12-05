use serde::Serialize;

#[derive(Serialize)]
pub struct Options {
    repeat_actions: bool,
    skip_impossible_moves: bool,
    wait_until_done: bool,
}

impl Options {
    pub const fn new(options: u8) -> Self {
        Self {
            repeat_actions:         options & 0b00000001 != 0,
            skip_impossible_moves:  options & 0b00000010 != 0,
            wait_until_done:        options & 0b00000100 != 0,
        }
    }
}
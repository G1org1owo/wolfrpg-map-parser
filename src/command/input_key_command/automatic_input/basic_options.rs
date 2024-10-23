use serde::Serialize;

#[derive(Serialize)]
pub struct BasicOptions {
    input_ok: bool,
    input_cancel: bool,
    input_subkey: bool,
    down: bool,
    left: bool,
    right: bool,
    up: bool,
}

impl BasicOptions {
    pub fn new(options: u8) -> Self {
        Self {
            input_ok:       options & 0b00000001 != 0,
            input_cancel:   options & 0b00000010 != 0,
            input_subkey:   options & 0b00000100 != 0,
            down:           options & 0b00010000 != 0,
            left:           options & 0b00100000 != 0,
            right:          options & 0b01000000 != 0,
            up:             options & 0b10000000 != 0,
        }
    }
}
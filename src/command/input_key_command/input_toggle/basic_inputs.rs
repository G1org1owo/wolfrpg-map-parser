use serde::Serialize;

#[derive(Serialize)]
pub struct BasicInputs {
    ok: bool,
    cancel: bool,
    sub: bool,
    down: bool,
    left: bool,
    right: bool,
    up: bool
}

impl BasicInputs {
    pub fn new(inputs: u8) -> Self {
        Self {
            ok:     inputs & 0b00000001 != 0,
            cancel: inputs & 0b00000010 != 0,
            sub:    inputs & 0b00000100 != 0,
            down:   inputs & 0b00010000 != 0,
            left:   inputs & 0b00100000 != 0,
            right:  inputs & 0b01000000 != 0,
            up:     inputs & 0b10000000 != 0,
        }
    }
}
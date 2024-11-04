use serde::Serialize;

#[derive(Serialize)]
pub struct Options {
    no_down: bool,
    no_left: bool,
    no_right: bool,
    no_up: bool,
    above_hero: bool,
    half_transparent: bool,
    counter: bool,
    match_lower_layer: bool,
}

impl Options {
    pub fn new(options: u32) -> Self {
        Self {
            no_down:            options & 0b00000001 != 0,
            no_left:            options & 0b00000010 != 0,
            no_right:           options & 0b00000100 != 0,
            no_up:              options & 0b00001000 != 0,
            above_hero:         options & 0b00010000 != 0,
            half_transparent:   options & 0b01000000 != 0,
            counter:            options & 0b10000000 != 0,

            match_lower_layer:  (options >> 8) & 0b00000010 != 0,
        }
    }
}
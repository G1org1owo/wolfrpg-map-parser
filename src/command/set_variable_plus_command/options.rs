use serde::Serialize;

#[derive(Serialize)]
pub struct Options {
    bind_result: bool,
    use_variable_as_reference: bool,
    precise_position: bool
}

impl Options {
    pub fn new(options: u8) -> Self {
        Options {
            bind_result:                options & 0b00000001 != 0,
            use_variable_as_reference:  options & 0b00010000 != 0,
            precise_position:           options & 0b00100000 != 0,
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
pub struct Options {
    bind_result: bool,
    real_number_calculation: bool,
    left_not_variable: bool,
    right_not_variable: bool,
    use_variable_as_reference: bool,
    use_left_as_reference: bool,
    use_right_as_reference: bool,
}

impl Options {
    pub fn new(options: u8) -> Self {
        Self {
            bind_result:                options & 0b00000001 != 0,
            real_number_calculation:    options & 0b00000010 != 0,
            left_not_variable:          options & 0b00000100 != 0,
            right_not_variable:         options & 0b00001000 != 0,
            use_variable_as_reference:  options & 0b00010000 != 0,
            use_left_as_reference:      options & 0b00100000 != 0,
            use_right_as_reference:     options & 0b01000000 != 0,
        }
    }
}
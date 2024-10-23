use serde::Serialize;

#[derive(Serialize)]
pub struct KeyOptions {
    wait_for_input: bool
}

impl KeyOptions {
    pub fn new(options: u8) -> Self {
        Self {
            wait_for_input: options & 0b10000000 != 0
        }
    }
}
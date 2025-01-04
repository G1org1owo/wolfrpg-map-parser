use crate::command::input_key_command::input_key::basic_options::BasicOptions;
use serde::Serialize;

#[derive(Serialize)]
pub struct Basic {
    options: BasicOptions
}

impl Basic {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u8 = bytes[offset];
        let options: BasicOptions = BasicOptions::new(options);
        offset += 1;

        offset += 1; // input_type

        offset += 2; // Padding

        (offset, Self {
            options
        })
    }

    pub fn options(&self) -> &BasicOptions {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut BasicOptions {
        &mut self.options
    }
}
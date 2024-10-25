use serde::Serialize;
use crate::command::picture_command::colors::Colors;

#[derive(Serialize)]
pub struct ColorsState {
    colors: Colors
}

impl ColorsState {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let colors: u8 = bytes[offset];
        let colors: Colors = Colors::new(colors);
        offset += 1;

        (offset, Self {
            colors
        })
    }
}
use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::effect_command::scroll_screen::options::Options;

mod options;
mod scroll_operation;
mod scroll_speed;

#[derive(Serialize)]
pub struct ScrollScreen {
    options: Options,
    x: u32,
    y: u32
}

impl ScrollScreen {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset+4]);
        let options: Options = Options::new(options);
        offset += 4;

        let x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            options,
            x,
            y
        })
    }
}
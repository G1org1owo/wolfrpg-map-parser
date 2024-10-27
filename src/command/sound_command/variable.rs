use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::sound_command::operation::Operation;
use crate::command::sound_command::options::Options;

#[derive(Serialize)]
pub struct Variable {
    delay_playback: Option<u32>,
    fade_time: Option<u32>,
    variable: u32,
    start_time: u32,
}

impl Variable {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let mut offset: usize = 0;

        let value: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let (delay_playback, fade_time): (Option<u32>, Option<u32>) = match *options.operation() {
            Operation::SetSE => (Some(value), None),
            _ => (None, Some(value))
        };

        let variable: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let start_time: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 2; // Padding

        (offset, Self {
            delay_playback,
            fade_time,
            variable,
            start_time
        })
    }
}
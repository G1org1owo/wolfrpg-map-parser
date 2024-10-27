use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};
use crate::command::sound_command::operation::Operation;
use crate::command::sound_command::options::Options;

#[derive(Serialize)]
pub struct Filename {
    delay_playback: Option<u32>,
    fade_time: Option<u32>,
    variable: u32,
    start_time: u32,
    volume: u32,
    tempo: u32,
    loop_point: Option<u32>,
    sound_filename: String
}

impl Filename {
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

        let volume: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let tempo: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let loop_point: Option<u32> = match *options.operation() {
            Operation::SetSE => None,
            _ => {
                let loop_point = as_u32_le(&bytes[offset..offset + 4]);
                offset += 4;

                Some(loop_point)
            }
        };

        offset += 1; // Padding

        offset += 1; // is_sound_string should always be true in this variant

        let sound_length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
        offset += 4;
        let sound_filename: String = as_string(bytes, offset, sound_length);
        offset += sound_length;

        (offset, Self {
            delay_playback,
            fade_time,
            variable,
            start_time,
            volume,
            tempo,
            loop_point,
            sound_filename
        })
    }
}
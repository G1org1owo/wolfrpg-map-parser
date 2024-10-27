mod options;
mod effect_type;
mod effect_target;
mod character_effect_type;
mod map_effect_type;
mod picture_effect_type;

use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::effect_command::base::options::Options;

#[derive(Serialize)]
pub struct Base {
    options: Options,
    duration: u32,
    target: u32,
    range: u32,
    value1: u32,
    value2: u32,
    value3: u32
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let options: Options = Options::new(options);
        offset += 4;

        let duration: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let target: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let range: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value1: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value2: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let value3: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            options,
            duration,
            target,
            range,
            value1,
            value2,
            value3
        })
    }
}
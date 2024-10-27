use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::effect_command::shake_type::ShakeType;

#[derive(Serialize)]
pub struct MapShake {
    power: u8,
    speed: u8,
    shake_type: ShakeType,
    duration: u32,
}

impl MapShake {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let power: u8 = (options & 0x0f) as u8;
        let speed: u8 = ((options & 0xf0) >> 4) as u8;

        let shake_type: u8 = ((options >> 8) & 0xff) as u8;
        let shake_type: ShakeType = ShakeType::new(shake_type);

        let duration: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            power,
            speed,
            shake_type,
            duration
        })
    }
}
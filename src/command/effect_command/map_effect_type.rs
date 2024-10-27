use serde::Serialize;

#[derive(Serialize)]
pub enum MapEffectType {
    Zoom    = 0x00,
    Shake   = 0x01,
    Unknown
}

impl MapEffectType {
    pub const fn new(effect_type: u8) -> Self {
        match effect_type {
            0x00 => MapEffectType::Zoom,
            0x01 => MapEffectType::Shake,
            _ => MapEffectType::Unknown
        }
    }
}
#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Transition {
    None    = 0x00,
    NoFade  = 0x01,
    Fade    = 0x02,
    Unknown,
}

impl Transition {
    pub const fn new(transition: u8) -> Self {
        match transition {
            0x00 => Transition::None,
            0x01 => Transition::NoFade,
            0x02 => Transition::Fade,
            _ => Transition::Unknown,
        }
    }
}
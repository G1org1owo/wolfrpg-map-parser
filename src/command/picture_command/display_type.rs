#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum DisplayType {
    LoadFile            = 0x00,
    Move                = 0x01,
    Erase               = 0x02,
    DelayReset          = 0x03,
    LoadFileByStringVar = 0x10,
    ShowStringAsPicture = 0x20,
    Window              = 0x30,
    WindowByStringVar   = 0x40,
    Unknown
}

impl DisplayType {
    pub const fn new(display_type: u8) -> Self {
        match display_type {
            0x00 => DisplayType::LoadFile,
            0x01 => DisplayType::Move,
            0x02 => DisplayType::Erase,
            0x03 => DisplayType::DelayReset,
            0x10 => DisplayType::LoadFileByStringVar,
            0x20 => DisplayType::ShowStringAsPicture,
            0x30 => DisplayType::Window,
            0x40 => DisplayType::WindowByStringVar,
            _ => DisplayType::Unknown
        }
    }
}
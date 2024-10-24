use serde::Serialize;

#[derive(Serialize)]
pub enum Zoom {
    Normal      = 0x00,
    Different   = 0x03,
    Same        = 0x04,
    Unknown
}

impl Zoom {
    pub const fn new(zoom: u8) -> Self {
        match zoom {
            0x00 => Zoom::Normal,
            0x03 => Zoom::Different,
            0x04 => Zoom::Same,
            _ => Zoom::Unknown
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
pub enum ScrollOperation {
    MoveScreen      = 0x00,
    BackToHero      = 0x01,
    LockScroll      = 0x02,
    UnlockScroll    = 0x03,
    Unknown
}

impl ScrollOperation {
    pub const fn new(operation: u8) -> Self {
        match operation {
            0x00 => ScrollOperation::MoveScreen,
            0x01 => ScrollOperation::BackToHero,
            0x02 => ScrollOperation::LockScroll,
            0x03 => ScrollOperation::UnlockScroll,
            _ => ScrollOperation::Unknown
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
pub enum Operation {
    SetBGM  = 0x00,
    SetBGS  = 0x01,
    SetSE   = 0x02,
    Unknown
}

impl Operation {
    pub const fn new(operation: u8) -> Self {
        match operation {
            0x00 => Operation::SetBGM,
            0x01 => Operation::SetBGS,
            0x02 => Operation::SetSE,
            _ => Operation::Unknown
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
pub enum DBOperationType {
    Write = 0b00000000,
    Read  = 0b00000001,
    Unknown
}

impl DBOperationType {
    pub const fn new(operation: u8) -> Self {
        match operation {
            0b00000000 => DBOperationType::Write,
            0b00000001 => DBOperationType::Read,
            _ => DBOperationType::Unknown
        }
    }
}
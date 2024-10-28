use serde::Serialize;

#[derive(Serialize)]
pub enum Operation {
    Save = 0x00000000,
    Load = 0x00000001,
    Unknown
}

impl Operation {
    pub const fn new(operation: u32) -> Self {
        match operation {
            0x00000000 => Operation::Save,
            0x00000001 => Operation::Load,
            _ => Operation::Unknown
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
pub enum VariableType {
    StringVariable      = 0b00000000,
    VariableReference   = 0b00000001,
    Unknown
}

impl VariableType {
    pub const fn new(variable_type: u8) -> Self {
        match variable_type {
            0b00000000 => VariableType::StringVariable,
            0b00000001 => VariableType::VariableReference,
            _ => VariableType::Unknown
        }
    }
}
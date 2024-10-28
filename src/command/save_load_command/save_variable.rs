use serde::Serialize;
use crate::command::save_load_command::parser::parse_variable_fields;

#[derive(Serialize)]
pub struct SaveVariable {
    source_variable: u32,
    save_number: u32,
    target_variable: u32,
    source_is_pointer: bool
}

impl SaveVariable {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let (mut offset, (source_variable, save_number, target_variable, source_is_pointer))
            : (usize, (u32, u32, u32, bool)) = parse_variable_fields(bytes);

        offset += 3; // Command end signature

        (offset, Self {
            source_variable,
            save_number,
            target_variable,
            source_is_pointer
        })
    }
}
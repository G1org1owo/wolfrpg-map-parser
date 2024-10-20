use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::set_variable_plus_command::character_field::CharacterField;

#[derive(Serialize)]
pub struct Character {
    unknown1: u16,
    character: u32,
    field: CharacterField,
}

impl Character {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let unknown1: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        let character: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let field: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let field: CharacterField = CharacterField::new(field);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            unknown1,
            character,
            field,
        })
    }
}
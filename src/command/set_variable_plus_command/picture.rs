use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::set_variable_plus_command::picture_field::PictureField;

#[derive(Serialize)]
pub struct Picture {
    unknown1: u16,
    picture_number: u32,
    field: PictureField
}

impl Picture {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let unknown1 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        let picture_number: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let field: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let field: PictureField = PictureField::new(field);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Picture {
            unknown1,
            picture_number,
            field
        })
    }
}
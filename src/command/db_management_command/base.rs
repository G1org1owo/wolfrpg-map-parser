use serde::Serialize;
use crate::byte_utils::{as_string, as_u16_le, as_u32_le};
use crate::command::db_management_command::assignment::Assignment;
use crate::command::db_management_command::options::Options;

use crate::command::common::u32_or_string::U32OrString;

#[derive(Serialize)]
pub struct Base {
    db_type: U32OrString, // name for table?
    data: U32OrString,    // name for tuple?
    field: U32OrString,
    assignment: Assignment,
    options: Options,
    unknown1: u16,
    value: u32,
    unknown2: u8,
}

impl Base {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let db_type: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let data: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let field: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let assignment: u8 = bytes[offset];
        let assignment: Assignment = Assignment::new(assignment);

        let options: u8 = bytes[offset+1];
        let options: Options = Options::new(options);

        offset += 2;

        let unknown1: u16 = as_u16_le(&bytes[offset..offset+2]);
        offset += 2;

        let value: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown2: u8 = bytes[offset];
        offset += 1;

        offset += 1; // String count, should always be 4

        offset += 5; // In this variant, value should always be a number

        let db_type_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let db_type_string: String = as_string(bytes, offset, db_type_length);
        offset += db_type_length;

        let db_type: U32OrString = if db_type_string.len() != 0 {
            U32OrString::String(db_type_string)
        } else {
            U32OrString::U32(db_type)
        };

        let data_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let data_string: String = as_string(bytes, offset, data_length);
        offset += data_length;

        let data: U32OrString = if data_string.len() != 0 {
            U32OrString::String(data_string)
        } else {
            U32OrString::U32(data)
        };

        let field_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let field_string: String = as_string(bytes, offset, field_length);
        offset += field_length;

        let field: U32OrString = if field_string.len() != 0 {
            U32OrString::String(field_string)
        } else {
            U32OrString::U32(field)
        };

        offset += 1; // Command end signature

        (offset, Self {
            db_type,
            data,
            field,
            assignment,
            options,
            unknown1,
            value,
            unknown2,
        })
    }
}
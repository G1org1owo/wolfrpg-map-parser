use serde::Serialize;
use crate::byte_utils::{as_string, as_u16_le, as_u32_le};
use crate::command::common::value_or_name::ValueOrName;
use crate::command::db_management_command::assignment::Assignment;
use crate::command::db_management_command::options::Options;

#[derive(Serialize)]
pub struct String {
    db_type: ValueOrName, // name for table?
    data: ValueOrName,    // name for tuple?
    field: ValueOrName,
    assignment: Assignment,
    options: Options,
    unknown1: u16,
    unknown2: u8,
    value: std::string::String,
}

impl String {
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

        let unknown2: u8 = bytes[offset];
        offset += 1;

        offset += 1; // String count, should always be 4

        let value_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let value: std::string::String = as_string(bytes, offset, value_length);
        offset += value_length;

        let db_type_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let db_type_string: std::string::String = as_string(bytes, offset, db_type_length);
        offset += db_type_length;

        let db_type: ValueOrName = if db_type_string.len() != 0 {
            ValueOrName::Name(db_type_string)
        } else {
            ValueOrName::Value(db_type)
        };

        let data_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let data_string: std::string::String = as_string(bytes, offset, data_length);
        offset += data_length;

        let data: ValueOrName = if data_string.len() != 0 {
            ValueOrName::Name(data_string)
        } else {
            ValueOrName::Value(data)
        };

        let field_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let field_string: std::string::String = as_string(bytes, offset, field_length);
        offset += field_length;

        let field: ValueOrName = if field_string.len() != 0 {
            ValueOrName::Name(field_string)
        } else {
            ValueOrName::Value(field)
        };

        offset += 1; // Command end signature

        (offset, Self {
            db_type,
            data,
            field,
            assignment,
            options,
            unknown1,
            unknown2,
            value
        })
    }
}
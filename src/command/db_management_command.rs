use crate::byte_utils::{as_u32_le, parse_string};
use crate::command::common::u32_or_string::U32OrString;
use crate::command::db_management_command::assignment::Assignment;
use crate::command::db_management_command::options::Options;
use crate::command::db_management_command::state::State;
use serde::Serialize;

mod base;
mod options;
mod db_operation_type;
mod db_type;
mod assignment;
mod assignment_operator;
mod string;
mod csv;
mod state;

#[derive(Serialize)]
pub struct DBManagementCommand {
    db_type: U32OrString, // name for table?
    data: U32OrString,    // name for tuple?
    field: U32OrString,
    assignment: Assignment,
    options: Options,
    state: State
}

impl DBManagementCommand {
    fn parse(bytes: &[u8], parse_state: fn(&[u8]) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

        let db_type: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let data: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let field: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let assignment: u8 = bytes[offset];
        let assignment: Assignment = Assignment::new(assignment);
        offset += 1;

        let options: u8 = bytes[offset];
        let options: Options = Options::new(options);
        offset += 1;

        offset += 2; // padding

        let (bytes_read, state): (usize, State) = parse_state(&bytes[offset..]);
        offset += bytes_read;

        let (bytes_read, db_type_string): (usize, String) = parse_string(&bytes[offset..]);
        let db_type: U32OrString = Self::get_u32_or_string(db_type, db_type_string);
        offset += bytes_read;

        let (bytes_read, data_string): (usize, String) = parse_string(&bytes[offset..]);
        let data: U32OrString = Self::get_u32_or_string(data, data_string);
        offset += bytes_read;

        let (bytes_read, field_string): (usize, String) = parse_string(&bytes[offset..]);
        let field: U32OrString = Self::get_u32_or_string(field, field_string);
        offset += bytes_read;

        offset += 1; // Command end signature

        (offset, Self {
            db_type,
            data,
            field,
            assignment,
            options,
            state
        })
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_base)
    }

    pub fn parse_string(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_string)
    }

    pub fn parse_csv(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_csv)
    }

    fn get_u32_or_string(value: u32, string: String) -> U32OrString {
        if string.len() != 0 {
            U32OrString::String(string)
        } else {
            U32OrString::U32(value)
        }
    }
}
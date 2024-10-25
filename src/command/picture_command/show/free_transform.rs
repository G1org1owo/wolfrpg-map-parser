use crate::byte_utils::as_u32_le;
use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::free_transform_state::FreeTransformState;
use crate::command::picture_command::show::parser::{make_filename_and_string, parse_string_value};
use serde::Serialize;

#[derive(Serialize)]
pub struct FreeTransform {
    top_left_x: u32,
    top_left_y: u32,
    unknown1: [u8; 3],
    state: FreeTransformState,
    unknown2: u8,
    filename: Option<U32OrString>,
    string: Option<String>,
}

impl FreeTransform {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let mut offset: usize = 0;

        let top_left_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let top_left_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 4; // zoom
        offset += 4; // angle

        let filename_variable: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown1: [u8; 3] = bytes[offset..offset+3].try_into().unwrap();
        offset += 3;

        let (bytes_read, state): (usize, FreeTransformState)
            = FreeTransformState::parse(&bytes[offset..]);
        offset += bytes_read;

        let unknown2: u8 = bytes[offset];
        offset += 1;

        let (bytes_read, string_value): (usize, Option<String>)
            = parse_string_value(&bytes[offset..]);
        offset += bytes_read;

        let (filename, string): (Option<U32OrString>, Option<String>)
            = make_filename_and_string(string_value, Some(filename_variable), options);

        (offset, Self {
            top_left_x,
            top_left_y,
            unknown1,
            state,
            unknown2,
            filename,
            string
        })
    }
}
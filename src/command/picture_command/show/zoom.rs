use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::colors::Colors;
use crate::command::picture_command::show::parser::{make_filename_and_string, parse_color_values, parse_string_value};

#[derive(Serialize)]
pub struct Zoom {
    position_x: u32,
    position_y: u32,
    unknown1: [u8; 3],
    colors: Colors,
    delay: u32,
    range_count: u32,
    color_values: [u32; 3],
    zoom_height: u32,
    unknown2: u8,
    filename: Option<U32OrString>,
    string: Option<String>,
}

impl Zoom {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let mut offset: usize = 0;

        let position_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 4; // zoom
        offset += 4; // angle

        let filename_variable: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown1: [u8; 3] = bytes[offset..offset+3].try_into().unwrap();
        offset += 3;

        let colors: u8 = bytes[offset];
        let colors: Colors = Colors::new(colors);
        offset += 1;

        let delay: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let range_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let (bytes_read, color_values): (usize, [u32; 3])
            = parse_color_values(&bytes[offset..]);
        offset += bytes_read;

        let zoom_height: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown2: u8 = bytes[offset];
        offset += 1;

        let (bytes_read, string_value): (usize, Option<String>)
            = parse_string_value(&bytes[offset..]);
        offset += bytes_read;

        let (filename, string): (Option<U32OrString>, Option<String>)
            = make_filename_and_string(string_value, Some(filename_variable), options);

        (offset, Self {
            position_x,
            position_y,
            unknown1,
            colors,
            delay,
            range_count,
            color_values,
            zoom_height,
            unknown2,
            filename,
            string
        })
    }
}
use crate::byte_utils::{as_string, as_u32_le};
use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::display_type::DisplayType;
use crate::command::picture_command::options::Options;

pub fn parse_filename_variable(bytes: &[u8], options: &Options) -> (usize, Option<u32>) {
    let mut offset: usize = 0;
    let filename_variable: Option<u32> = match *options.display_type() {
        DisplayType::LoadFileByStringVar | DisplayType::WindowByStringVar => {
            let filename_variable = as_u32_le(&bytes[offset..offset+4]);
            offset += 4;

            Some(filename_variable)
        }

        _ => None
    };

    (offset, filename_variable)
}

pub fn parse_string_value(bytes: &[u8]) -> (usize, Option<String>) {
    let mut offset: usize = 0;

    let is_filename_string: bool = bytes[offset] != 0;
    offset += 1;

    let string_value: Option<String> = if is_filename_string {
        let string_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        let string: String = as_string(bytes, offset, string_length);
        offset += string_length;

        Some(string)
    } else {
        None
    };

    (offset, string_value)
}

pub fn make_filename_and_string(string_value: Option<String>, filename_variable: Option<u32>,
                                options: &Options) -> (Option<U32OrString>, Option<String>) {
    let (filename, string): (Option<String>, Option<String>) = match *options.display_type() {
        DisplayType::ShowStringAsPicture => (None, string_value),
        _ => (string_value, None)
    };

    let filename: Option<U32OrString> = match filename {
        Some(filename) => Some(U32OrString::String(filename)),
        None => match filename_variable {
            Some(filename_variable) => Some(U32OrString::U32(filename_variable)),
            None => None
        }
    };

    (filename, string)
}

pub fn parse_color_values(bytes: &[u8]) -> (usize, [u32; 3]) {
    let mut offset: usize = 0;

    let color1: u32 = as_u32_le(&bytes[offset..offset+4]);
    offset += 4;
    let color2: u32 = as_u32_le(&bytes[offset..offset+4]);
    offset += 4;
    let color3: u32 = as_u32_le(&bytes[offset..offset+4]);
    offset += 4;

    (offset, [
        color1,
        color2,
        color3
    ])
}
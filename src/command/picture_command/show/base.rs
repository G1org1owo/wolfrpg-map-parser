use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};
use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::display_type::DisplayType;
use crate::command::picture_command::options::Options;

#[derive(Serialize)]
pub struct Base {
    position_x: u32,
    position_y: u32,
    unknown1: u8,
    filename: Option<U32OrString>,
    string: Option<String>
}

impl Base {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let mut offset: usize = 0;

        let position_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 4; // zoom
        offset += 4; // angle

        let filename_variable: Option<u32> = match *options.display_type() {
            DisplayType::LoadFileByStringVar | DisplayType::WindowByStringVar => {
                let filename_variable = as_u32_le(&bytes[offset..offset+4]);
                offset += 4;

                Some(filename_variable)
            }

            _ => None
        };

        let unknown1: u8 = bytes[offset];
        offset += 1;

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

        let (filename, string): (Option<String>, Option<String>) = match *options.display_type() {
            DisplayType::ShowStringAsPicture => (None, string_value),
            _ => (string_value, None)
        };

        let filename: Option<U32OrString> = match filename_variable {
            Some(filename_variable) => Some(U32OrString::U32(filename_variable)),
            None => match filename {
                Some(filename) => Some(U32OrString::String(filename)),
                None => None
            }
        };

        (offset, Base{
            position_x,
            position_y,
            unknown1,
            filename,
            string
        })
    }
}
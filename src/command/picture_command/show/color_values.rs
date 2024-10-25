use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::color_values_fields::ColorValuesFields;
use crate::command::picture_command::show::parsable_state::ParsableState;
use serde::Serialize;

#[derive(Serialize)]
pub struct ColorValues {
    position_x: u32,
    position_y: u32,
    unknown1: [u8; 3],
    fields: ColorValuesFields,
    unknown2: u8,
    filename: Option<U32OrString>,
    string: Option<String>,
}

impl ColorValues {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
         let (offset, (position_x, position_y, unknown1, fields, unknown2, filename, string))
            : (usize, (u32, u32, [u8; 3], ColorValuesFields, u8, Option<U32OrString>,
                       Option<String>))
            = Self::parse_fields(bytes, options);

        (offset, Self {
            position_x,
            position_y,
            unknown1,
            fields,
            unknown2,
            filename,
            string
        })
    }
}

impl ParsableState<ColorValuesFields> for ColorValues { }
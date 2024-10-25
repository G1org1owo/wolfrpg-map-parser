use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::range_fields::RangeFields;
use serde::Serialize;
use crate::command::picture_command::show::parsable_state::ParsableState;

#[derive(Serialize)]
pub struct Range {
    position_x: u32,
    position_y: u32,
    unknown1: [u8; 3],
    fields: RangeFields,
    unknown2: u8,
    filename: Option<U32OrString>,
    string: Option<String>,
}

impl Range {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (offset, (position_x, position_y, unknown1, fields, unknown2, filename, string))
            : (usize, (u32, u32, [u8; 3], RangeFields, u8, Option<U32OrString>,
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

impl ParsableState<RangeFields> for Range { }
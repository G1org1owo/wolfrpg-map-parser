use crate::command::common::u32_or_string::U32OrString;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::free_transform_fields::FreeTransformFields;
use crate::command::picture_command::show::parsable_state::ParsableState;
use serde::Serialize;

#[derive(Serialize)]
pub struct FreeTransform {
    top_left_x: u32,
    top_left_y: u32,
    unknown1: [u8; 3],
    fields: FreeTransformFields,
    unknown2: u8,
    filename: Option<U32OrString>,
    string: Option<String>,
}

impl FreeTransform {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (offset, (top_left_x, top_left_y, unknown1, fields, unknown2, filename, string))
            : (usize, (u32, u32, [u8; 3], FreeTransformFields, u8, Option<U32OrString>,
                       Option<String>))
            = Self::parse_fields(bytes, options);

        (offset, Self {
            top_left_x,
            top_left_y,
            unknown1,
            fields,
            unknown2,
            filename,
            string
        })
    }
}

impl ParsableState<FreeTransformFields> for FreeTransform { }
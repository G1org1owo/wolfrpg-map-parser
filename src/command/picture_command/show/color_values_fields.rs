use crate::command::picture_command::show::parser::parse_color_values;
use crate::command::picture_command::show::range_fields::RangeFields;
use serde::Serialize;
use crate::command::picture_command::show::parsable_fields::ParsableFields;

#[derive(Serialize)]
pub struct ColorValuesFields {
    range_state: RangeFields,
    color_values: [u32; 3]
}

impl ParsableFields<ColorValuesFields> for ColorValuesFields {
    fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, range_state): (usize, RangeFields) = RangeFields::parse(bytes);
        offset += bytes_read;

        let (bytes_read, color_values): (usize, [u32; 3])
            = parse_color_values(&bytes[offset..]);
        offset += bytes_read;

        (offset, Self {
            range_state,
            color_values
        })
    }
}
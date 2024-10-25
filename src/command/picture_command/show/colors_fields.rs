use serde::Serialize;
use crate::command::picture_command::colors::Colors;
use crate::command::picture_command::show::parsable_fields::ParsableFields;

#[derive(Serialize)]
pub struct ColorsFields {
    colors: Colors
}

impl ParsableFields<ColorsFields> for ColorsFields {
    fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let colors: u8 = bytes[offset];
        let colors: Colors = Colors::new(colors);
        offset += 1;

        (offset, Self {
            colors
        })
    }
}
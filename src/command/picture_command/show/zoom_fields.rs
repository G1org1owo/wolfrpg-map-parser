use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::show::color_values_fields::ColorValuesFields;
use crate::command::picture_command::show::parsable_fields::ParsableFields;

#[derive(Serialize)]
pub struct ZoomFields {
    colors_state: ColorValuesFields,
    zoom_height: u32
}

impl ParsableFields<ZoomFields> for ZoomFields {
    fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, colors_state): (usize, ColorValuesFields) = ColorValuesFields::parse(bytes);
        offset += bytes_read;

        let zoom_height: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            colors_state,
            zoom_height
        })
    }
}
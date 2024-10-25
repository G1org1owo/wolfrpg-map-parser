use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::show::parsable_fields::ParsableFields;
use crate::command::picture_command::show::zoom_fields::ZoomFields;

#[derive(Serialize)]
pub struct FreeTransformFields {
    zoom_state: ZoomFields,
    top_right_x: u32,
    top_right_y: u32,
    bottom_left_x: u32,
    bottom_left_y: u32,
    bottom_right_x: u32,
    bottom_right_y: u32
}

impl ParsableFields<FreeTransformFields> for FreeTransformFields {
    fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, zoom_state): (usize, ZoomFields) = ZoomFields::parse(bytes);
        offset += bytes_read;

        let top_right_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let top_right_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let bottom_left_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let bottom_left_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let bottom_right_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let bottom_right_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            zoom_state,
            top_right_x,
            top_right_y,
            bottom_left_x,
            bottom_left_y,
            bottom_right_x,
            bottom_right_y
        })
    }
}
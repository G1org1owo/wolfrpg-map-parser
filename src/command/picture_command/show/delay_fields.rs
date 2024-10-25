use crate::byte_utils::as_u32_le;
use crate::command::picture_command::show::colors_fields::ColorsFields;
use serde::Serialize;
use crate::command::picture_command::show::parsable_fields::ParsableFields;

#[derive(Serialize)]
pub struct DelayFields {
    colors_state: ColorsFields,
    delay: u32
}

impl ParsableFields<DelayFields> for DelayFields {
    fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, colors_state): (usize, ColorsFields) = ColorsFields::parse(bytes);
        offset += bytes_read;

        let delay: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            colors_state,
            delay
        })
    }
}
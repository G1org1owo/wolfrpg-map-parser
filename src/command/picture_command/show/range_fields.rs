use crate::byte_utils::as_u32_le;
use crate::command::picture_command::show::delay_fields::DelayFields;
use serde::Serialize;
use crate::command::picture_command::show::parsable_fields::ParsableFields;

#[derive(Serialize)]
pub struct RangeFields {
    delay_state: DelayFields,
    range_count: u32
}

impl ParsableFields<RangeFields> for RangeFields {
    fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, delay_state): (usize, DelayFields) = DelayFields::parse(bytes);
        offset += bytes_read;

        let range_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            delay_state,
            range_count
        })
    }
}
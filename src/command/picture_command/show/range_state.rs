use crate::byte_utils::as_u32_le;
use crate::command::picture_command::show::delay_state::DelayState;
use serde::Serialize;

#[derive(Serialize)]
pub struct RangeState {
    delay_state: DelayState,
    range_count: u32
}

impl RangeState {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, delay_state): (usize, DelayState) = DelayState::parse(bytes);
        offset += bytes_read;

        let range_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            delay_state,
            range_count
        })
    }
}
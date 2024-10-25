use crate::byte_utils::as_u32_le;
use crate::command::picture_command::show::colors_state::ColorsState;
use serde::Serialize;

#[derive(Serialize)]
pub struct DelayState {
    colors_state: ColorsState,
    delay: u32
}

impl DelayState {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, colors_state): (usize, ColorsState) = ColorsState::parse(bytes);
        offset += bytes_read;

        let delay: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        (offset, Self {
            colors_state,
            delay
        })
    }
}
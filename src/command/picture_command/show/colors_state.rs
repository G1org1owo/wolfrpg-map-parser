use crate::command::picture_command::show::parser::parse_color_values;
use crate::command::picture_command::show::range_state::RangeState;
use serde::Serialize;

#[derive(Serialize)]
pub struct ColorsState {
    range_state: RangeState,
    color_values: [u32; 3]
}

impl ColorsState {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, range_state): (usize, RangeState) = RangeState::parse(bytes);
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
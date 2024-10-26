use serde::Serialize;
use crate::byte_utils::as_u32_le;

#[derive(Serialize)]
pub struct DelayReset {
    range_count: Option<u32>
}

impl DelayReset {
    pub fn parse(bytes: &[u8], range: bool) -> (usize, Self) {
        let mut offset: usize = 0;

        let range_count: Option<u32> = if range {
            let range_count: u32 = as_u32_le(&bytes[offset..offset+4]);
            offset += 4;

            Some(range_count)
        } else {
            None
        };

        (offset, Self {
            range_count
        })
    }
}
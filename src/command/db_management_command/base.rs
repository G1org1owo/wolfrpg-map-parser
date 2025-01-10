use crate::byte_utils::as_u32_le;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct Base {
    value: u32,
}

impl Base {
    pub(crate) fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let value: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        offset += 1; // padding

        offset += 1; // String count, should always be 4

        offset += 5; // In this variant, value should always be a number

        (offset, Self {
            value
        })
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}
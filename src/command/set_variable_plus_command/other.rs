use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::set_variable_plus_command::target::Target;

#[derive(Serialize)]
pub struct Other {
    unknown1: u16,
    target: Target
}

impl Other {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let unknown1: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        let target: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let target: Target = Target::new(target);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            unknown1,
            target
        })
    }
}
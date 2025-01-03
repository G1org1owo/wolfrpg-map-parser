use crate::byte_utils::as_u32_le;
use crate::command::set_variable_plus_command::target::Target;
use serde::Serialize;

#[derive(Serialize)]
pub struct Other {
    target: Target
}

impl Other {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        offset += 2;

        let target: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let target: Target = Target::new(target);
        offset += 4;

        (offset, Self {
            target
        })
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn target_mut(&mut self) -> &mut Target {
        &mut self.target
    }
}
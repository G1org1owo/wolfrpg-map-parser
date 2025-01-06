use crate::byte_utils::parse_string;
use serde::Serialize;

#[derive(Serialize)]
pub struct Label {
    label: String
}

impl Label {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;
        offset += 2; // padding + string_count which are always 0x0001

        let (bytes_read, label): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;

        offset += 1; // Command end

        (offset, Self {
            label
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }
    
    pub fn label_mut(&mut self) -> &mut String {
        &mut self.label
    }
}
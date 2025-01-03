use serde::Serialize;
use crate::command::Command;

#[derive(Serialize)]
pub struct Loop {
    commands: Vec<Command>,
}

impl Loop {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;
        offset += 3; // Command end signature

        let (bytes_read, mut commands_read, commands): (usize, u32, Vec<Command>)
            = Command::parse_multiple(&bytes[offset..]);
        offset += bytes_read;

        let _loop_end_signature: &[u8] = &bytes[offset..offset+8]; //TODO: Should be 01f20100 00000000
        offset += 8;
        commands_read += 1;

        (offset, commands_read, Self {
            commands
        })
    }
}
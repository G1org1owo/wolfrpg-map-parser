use serde::Serialize;
use crate::command::Command;

#[derive(Serialize)]
pub struct Loop {
    unknown1: [u8; 3],
    commands: Vec<Command>,
}

impl Loop {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;

        let unknown1: [u8; 3] = bytes[offset..offset + 3].try_into().unwrap();
        offset += 3;

        let (bytes_read, mut commands_read, commands): (usize, u32, Vec<Command>)
            = Self::parse_commands(&bytes[offset..]);
        offset += bytes_read;

        let _loop_end_signature: &[u8] = &bytes[offset..offset+8]; //TODO: Should be 01f20100 00000000
        offset += 8;
        commands_read += 1;

        (offset, commands_read, Self {
            unknown1,
            commands
        })
    }

    fn parse_commands(bytes: &[u8]) -> (usize, u32, Vec<Command>) {
        let mut offset: usize = 0;
        let mut command_count: u32 = 0;
        let mut commands: Vec<Command> = vec![];

        let mut exit: bool = false;

        while(!exit) {
            let (bytes_read, commands_read, command): (usize, u32, Command)
                = Command::parse(&bytes[offset..]);

            exit = match command {
                Command::Exit() => true,
                _ => false
            };

            offset += bytes_read;
            command_count += commands_read;
            commands.push(command);
        }

        (offset, command_count, commands)
    }
}
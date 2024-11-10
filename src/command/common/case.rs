use crate::byte_utils::{as_u32_be, as_u32_le};
use crate::command::common::case_type::CaseType;
use crate::command::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct Case {
    case_type: CaseType,
    case_id: u32,
    unknown1: [u8; 3],
    commands: Vec<Command>,
}

impl Case {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;

        let case_type: u32 = as_u32_be(&bytes[offset..offset+4]);
        let case_type: CaseType = CaseType::new(case_type);
        offset += 4;

        offset += 1; // Padding

        let case_id: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown1: [u8; 3] = bytes[offset..offset+3].try_into().unwrap();
        offset += 3;

        let mut commands: Vec<Command> = vec![];
        let mut command_count: u32 = 1; // Case counts as command
        let mut exit: bool = false;

        while(!exit) {
            let (bytes_read, commands_read, command): (usize, u32, Command)
                = Command::parse(&bytes[offset..]);

            exit = match command {
                Command::Exit() => true,
                _ => false
            };

            commands.push(command);
            offset += bytes_read;
            command_count += commands_read;
        }

        (offset, command_count, Self {
            case_type,
            case_id,
            unknown1,
            commands,
        })
    }

    pub fn parse_multiple(bytes: &[u8], case_count: usize) -> (usize, u32, Vec<Case>) {
        let mut cases: Vec<Case> = Vec::with_capacity(case_count);
        let mut offset: usize = 0;
        let mut commands: u32 = 0;

        for _i in 0..case_count {
            let (bytes_read, commands_read, case): (usize, u32, Case) = Self::parse(&bytes[offset..]);
            cases.push(case);
            offset += bytes_read;
            commands += commands_read;
        }

        (offset, commands, cases)
    }
}
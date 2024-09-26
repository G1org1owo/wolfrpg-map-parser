use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_be, as_u32_le};
use crate::command::Command;

#[derive(Serialize)]
enum CaseType {
    Choice,
    Extra,
    Cancel
}

const CHOICE_CASE: u32 = 0x02910100;
const EXTRA_CASE: u32 = 0x02920100;
const CANCEL_CASE: u32 = 0x02A50100;

#[derive(Serialize)]
pub struct Case {
    case_type: CaseType,
    unknown1: u8,
    case_id: u8,
    unknown2: u16,
    unknown3: u32,
    commands: Vec<Command>,
}

impl Case {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;

        let case_type: CaseType = match as_u32_be(&bytes[offset..offset+4]) {
            CHOICE_CASE => Ok(CaseType::Choice),
            EXTRA_CASE => Ok(CaseType::Extra),
            CANCEL_CASE => Ok(CaseType::Cancel),
            _ => Err("Not a valid case")
        }.expect(format!(
            "Could not interpret case {:08x}",
            as_u32_be(&bytes[offset..offset+4])
        ).as_str());
        offset += 4;

        let unknown1: u8 = bytes[offset];
        let case_id: u8 = bytes[offset+1];
        let unknown2: u16 = as_u16_le(&bytes[offset+2..offset+4]);
        let unknown3: u32 = as_u32_le(&bytes[offset+4..offset+8]);
        offset += 8;

        let mut commands: Vec<Command> = vec![];
        let mut command_count: u32 = 0;
        loop {
            let (bytes_read, commands_read, command): (usize, u32, Command)
                = Command::parse(&bytes[offset..]);

            commands.push(command);
            offset += bytes_read;
            command_count += commands_read;

            if let Command::Exit() = commands.last().unwrap() {
                break;
            };
        }

        (offset, command_count, Self {
            case_type,
            unknown1,
            case_id,
            unknown2,
            unknown3,
            commands,
        })
    }
}
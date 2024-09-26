use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::command::Command;

#[derive(Serialize)]
enum CaseType {
    Choice,
    Extra
}

#[derive(Serialize)]
pub struct Case {
    case_type: CaseType,
    unknown1: u8,
    case_id: u8,
    unknown2: u16,
    unknown3: u32,
    commands: Vec<Command>,
    unknown4: u32,
    unknown5: u32,
}

impl Case {
    fn case_id () {

    }

    pub fn parse_choice (bytes: &[u8]) -> (usize, Case) {
        let (offset, mut case): (usize, Case) = Self::parse(bytes, CaseType::Choice);

        (offset, case)
    }

    pub fn parse_extra (bytes: &[u8]) -> (usize, Case) {
        let (offset, mut case): (usize, Case) = Self::parse(bytes, CaseType::Extra);

        (offset, case)
    }

    fn parse(bytes: &[u8], case_type: CaseType) -> (usize, Self) {
        let mut offset = 0;

        let unknown1: u8 = bytes[offset];
        let case_id: u8 = bytes[offset+1];
        let unknown2: u16 = as_u16_le(&bytes[offset+2..offset+4]);
        let unknown3: u32 = as_u32_le(&bytes[offset+4..offset+8]);
        offset += 8;

        let mut commands: Vec<Command> = vec![];
        loop {
            let (bytes_read, command): (usize, Command) = Command::parse(&bytes[offset..]);
            commands.push(command);
            offset += bytes_read;

            if let Command::Exit() = commands.last().unwrap() {
                break;
            };
        }

        let unknown4: u32 = as_u32_le(&bytes[offset..offset+4]);
        let unknown5: u32 = as_u32_le(&bytes[offset+4..offset+8]);
        offset += 8;

        (offset, Self {
            case_type,
            unknown1,
            case_id,
            unknown2,
            unknown3,
            commands,
            unknown4,
            unknown5,
        })
    }
}
use serde::Serialize;
use crate::byte_utils::{as_string, as_u16_le, as_u32_le};
use crate::case::Case;

#[derive(Serialize)]
enum Cancel {
    Choice(u8),
    Separate,
    No,
}

#[derive(Serialize)]
pub struct ShowChoiceCommand {
    cancel_case: Cancel,
    selected_choices: u8, // only take lower half for actual choices
    extra_cases: u8, // bitmap
    unknown1: u16,
    unknown2: u8,
    choice_count: u8,
    choices: Vec<String>,
    cases: Vec<Case>,
}

impl ShowChoiceCommand {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self){
        let mut offset: usize = 0;

        let selected_choices: u8 = bytes[offset];
        let extra_cases: u8 = bytes[offset+1];
        let unknown1: u16 = as_u16_le(&bytes[offset+2..offset+4]);
        let unknown2: u8 = bytes[offset+4];
        let choice_count: u8 = bytes[offset+5];
        offset += 6;

        let cancel_case: u8 = (selected_choices >> 4) & 0b00001111;
        let cancel_case: Cancel = match cancel_case {
            0 => Cancel::Separate,
            1 => Cancel::No,
            _ => Cancel::Choice(cancel_case - 2)
        };

        let mut choices: Vec<String> = vec![];

        for _i in 0..choice_count {
            let (bytes_read, choice): (usize, String) = Self::parse_choice(&bytes[offset..]);
            choices.push(choice);
            offset += bytes_read;
        }

        offset += 1; // should be 0x00 to indicate end of choices

        let (bytes_read, mut commands_read, cases): (usize, u32, Vec<Case>) = Self::parse_cases(
            &bytes[offset..],
            choice_count + Self::extra_cases_count(extra_cases, &cancel_case)
        );
        offset += bytes_read;

        let end_command = &bytes[offset..offset+8];
        offset += 8;
        commands_read += 1; // This should be some sort of command end signature I guess

        (offset, commands_read, Self {
            cancel_case,
            selected_choices,
            extra_cases,
            unknown1,
            unknown2,
            choice_count,
            choices,
            cases,
        })
    }

    fn parse_choice(bytes: &[u8]) -> (usize, String) {
        let length: usize = as_u32_le(&bytes[..4]) as usize;
        let choice: String = as_string(bytes, 4, length);

        (length + 4, choice)
    }

    fn parse_cases(bytes: &[u8], case_count: u8) -> (usize, u32, Vec<Case>) {
        let mut cases: Vec<Case> = vec![];
        let mut offset: usize = 0;
        let mut commands: u32 = 0;

        for _i in 0..case_count {
            let (bytes_read, commands_read, case): (usize, u32, Case) = Case::parse(&bytes[offset..]);
            cases.push(case);
            offset += bytes_read;
            commands += commands_read;
            commands += 1; // case counts as command
        }

        (offset, commands, cases)
    }

    fn extra_cases_count(extra_cases: u8, cancel: &Cancel) -> u8 {
        let base_cases: u8 = match cancel {
            Cancel::Separate => 1,
            _ => 0
        };

        base_cases +
        (extra_cases >> 0) & 0x1 +
        (extra_cases >> 1) & 0x1 +
        (extra_cases >> 2) & 0x1
    }
}

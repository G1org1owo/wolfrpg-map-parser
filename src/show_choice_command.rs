use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::case::Case;

#[derive(Serialize)]
enum Cancel {
    Choice(u8),
    Separate,
    No,
}

#[derive(Serialize)]
pub struct ShowChoiceCommand {
    unknown1: u8,
    cancel_case: Cancel,
    selected_choices: u8, // only take lower half for actual choices
    extra_cases: u8, // bitmap
    unknown2: u16,
    unknown3: u8,
    choice_count: u8,
    choices: Vec<String>,
    cases: Vec<Case>,
    unknown4: u32,
    unknown5: u32,
}

impl ShowChoiceCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self){
        let mut offset: usize = 0;

        let unknown1: u8 = bytes[offset];
        let selected_choices: u8 = bytes[offset+1];
        let extra_cases: u8 = bytes[offset+2];
        let unknown2: u16 = as_u16_le(&bytes[offset+3..offset+5]);
        let unknown3: u8 = bytes[offset+5];
        let choice_count: u8 = bytes[offset+6];
        offset += 7;

        let cancel_case: u8 = (selected_choices >> 4) & 0b00001111;
        let cancel_case: Cancel = match cancel_case {
            0 => Cancel::Separate,
            1 => Cancel::No,
            _ => Cancel::Choice(cancel_case)
        };

        let mut choices: Vec<String> = vec![];

        for _i in 0..choice_count {
            let (bytes_read, choice): (usize, String) = Self::parse_choice(&bytes[offset..]);
            choices.push(choice);
            offset += bytes_read;
        }

        offset += 1; // should be 0x00 to indicate end of choices

        let (bytes_read, cases): (usize, Vec<Case>) = Self::parse_cases(
            &bytes[offset..],
            choice_count + Self::extra_cases_count(extra_cases, &cancel_case)
        );
        offset += bytes_read;

        let unknown4: u32 = as_u32_le(&bytes[offset..offset+4]);
        let unknown5: u32 = as_u32_le(&bytes[offset+4..offset+8]);
        offset += 8;

        (offset, Self {
            unknown1,
            cancel_case,
            selected_choices,
            extra_cases,
            unknown2,
            unknown3,
            choice_count,
            choices,
            cases,
            unknown4,
            unknown5,
        })
    }

    fn parse_choice(bytes: &[u8]) -> (usize, String) {
        let length: usize = as_u32_le(&bytes[..4]) as usize;
        let choice: String = String::from_utf8(bytes[4.. length+4-1].to_vec()).unwrap();

        (length + 4, choice)
    }

    fn parse_cases(bytes: &[u8], case_count: u8) -> (usize, Vec<Case>) {
        let mut cases: Vec<Case> = vec![];
        let mut offset: usize = 0;
        let mut cases: Vec<Case> = Vec::new();

        for _i in 0..case_count {
            let (bytes_read, case): (usize, Case) = Case::parse(&bytes[offset..]);
            cases.push(case);
            offset += bytes_read;
            commands += 1; // case counts as command
        }

        (offset, cases)
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

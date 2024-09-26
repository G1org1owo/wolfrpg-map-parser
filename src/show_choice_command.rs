use serde::Serialize;
use crate::byte_utils::{as_u16_le, as_u32_le};
use crate::case::Case;

#[derive(Serialize)]
pub struct ShowChoiceCommand {
    unknown1: u8,
    selected_choices: u8, // half for selected, half for actual choices
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

        let mut choices: Vec<String> = vec![];

        for _i in 0..choice_count {
            let (bytes_read, choice): (usize, String) = Self::parse_choice(&bytes[offset..]);
            choices.push(choice);
            offset += bytes_read;
        }

        let mut cases: Vec<Case> = vec![];

        for _i in 0..choice_count {
            let (bytes_read, case): (usize, Case) = Case::parse_choice(&bytes[offset..]);
            cases.push(case);
            offset += bytes_read;
        }

        for _i in 0..extra_cases {
            let (bytes_read, case): (usize, Case) = Case::parse_extra(&bytes[offset..]);
            cases.push(case);
            offset += bytes_read;
        }

        let unknown4: u32 = as_u32_le(&bytes[offset..offset+4]);
        let unknown5: u32 = as_u32_le(&bytes[offset+4..offset+8]);
        offset += 8;

        (offset, Self {
            unknown1,
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
        let choice: String = String::from_utf8(bytes[4.. length+4].to_vec()).unwrap();

        (length + 4, choice)
    }

    fn parse_cases(bytes: &[u8], case_count: usize) -> (usize, Vec<Case>) {
        let mut offset: usize = 0;
        let mut cases: Vec<Case> = Vec::new();

        for _i in 0..case_count {
            let (bytes_read, case): (usize, Case) = Case::parse_choice(&bytes[offset..]);

            offset += bytes_read;
            cases.push(case);
        }

        (offset, cases)
    }
}

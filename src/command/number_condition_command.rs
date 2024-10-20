use serde::Serialize;
use crate::command::common::case::Case;
use crate::command::number_condition_command::condition::Condition;

mod condition;
mod operator;
mod compare_operator;

#[derive(Serialize)]
pub struct NumberConditionCommand {
    case_count: u8,
    else_case: bool,
    conditions: Vec<Condition>,
    unknown1: [u8; 3],

    cases: Vec<Case>,
}

impl NumberConditionCommand {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;

        let (case_count, else_case): (u8, bool) = Self::parse_case_count(bytes[offset]);
        offset += 1;

        offset += 3; // padding

        let (bytes_read, conditions): (usize, Vec<Condition>)
            = Self::parse_conditions(&bytes[offset..], case_count as usize);

        offset += bytes_read;

        let unknown1: [u8; 3] = bytes[offset..offset+3].try_into().unwrap();
        offset += 3;

        let (bytes_read, mut commands_read, cases): (usize, u32, Vec<Case>)
            = Self::parse_cases(&bytes[offset..], case_count as usize, else_case);

        offset += bytes_read;

        let end_command = &bytes[offset..offset+8];
        offset += 8; // TODO: Throw error if it's not 0x01f30000 0x00000000
        commands_read += 1;

        (offset, commands_read, Self {
            case_count,
            else_case,
            conditions,
            unknown1,
            cases
        })
    }

    fn parse_case_count(cases: u8) -> (u8, bool) {
        (cases & 0x0f, cases & 0b00010000 != 0)
    }

    fn parse_conditions(bytes: &[u8], condition_count: usize) -> (usize, Vec<Condition>) {
        let mut offset: usize = 0;
        let mut conditions: Vec<Condition> = Vec::with_capacity(condition_count);

        for _ in 0..condition_count {
            let (bytes_read, condition): (usize, Condition)
                = Condition::parse(&bytes[offset..]);
            conditions.push(condition);
            offset += bytes_read;
        }

        (offset, conditions)
    }

    fn parse_cases(bytes: &[u8], case_count: usize, else_case: bool) -> (usize, u32, Vec<Case>) {
        let mut offset: usize = 0;
        let mut command_count: u32 = 0;
        let mut cases: Vec<Case> = Vec::with_capacity(case_count);

        let else_case: usize = if else_case { 1 } else { 0 };

        for _ in 0..case_count + else_case {
            let (bytes_read, commands_read, case): (usize, u32, Case)
                = Case::parse(&bytes[offset..]);

            offset += bytes_read;
            command_count += commands_read;
            cases.push(case);
        }

        (offset, command_count, cases)
    }
}
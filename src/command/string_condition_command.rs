mod operator;
mod compare_operator;
mod condition;

use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_vec, as_u32_le};
use crate::command::common::case::Case;
use crate::command::common::u32_or_string::U32OrString;
use crate::command::string_condition_command::condition::Condition;
use crate::command::string_condition_command::operator::Operator;

#[derive(Serialize)]
pub struct StringConditionCommand {
    case_count: u8,
    else_case: bool,
    conditions: Vec<Condition>,
    cases: Vec<Case>
}

impl StringConditionCommand {
    pub fn parse(bytes: &[u8], signature: u32) -> (usize, u32, Self) {
        let mut offset: usize = 0;

        let (case_count, else_case): (u8,bool) = Self::parse_case_count(bytes[offset]);
        offset += 1;

        offset += 3; // Padding

        print!("Offset: {:0x}", offset);

        let variables: Vec<u32> = as_u32_vec(&bytes[offset..offset + (4 * case_count) as usize]);
        offset += 4 * case_count as usize;

        let value_count:usize = Self::value_count(signature, case_count as u32);
        let values: Vec<u32> = as_u32_vec(&bytes[offset..offset + (4 * value_count)]);
        offset += 4 * value_count;

        offset += 1; // Padding;

        let condition_count: u8 = bytes[offset];
        offset += 1;

        let (bytes_read, conditions): (usize, Vec<String>)
            = Self::parse_conditions(&bytes[offset..], condition_count);
        offset += bytes_read;

        offset += 1; // Conditions end

        let conditions: Vec<Condition> = Self::make_conditions(variables, values, conditions);

        let (bytes_read, mut commands_read, cases): (usize, u32, Vec<Case>) = Self::parse_cases(
            &bytes[offset..], case_count as usize, else_case
        );

        offset += bytes_read;

        let end_command = &bytes[offset..offset+8];
        offset += 8; // TODO: Throw error if it's not 0x01f30000 0x00000000
        commands_read += 1;

        (offset, commands_read, Self {
            case_count,
            else_case,
            conditions,
            cases
        })
    }

    fn parse_case_count(cases: u8) -> (u8, bool) {
        (cases & 0x0f, cases & 0b00010000 != 0)
    }

    fn value_count(signature: u32, case_count: u32) -> usize {
        ((signature >> 24) - 2 - case_count) as usize
    }

    fn parse_conditions(bytes: &[u8], condition_count: u8) -> (usize, Vec<String>) {
        let mut offset: usize = 0;
        let mut strings: Vec<String> = Vec::new();

        for _ in 0..condition_count {
            let length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
            offset += 4;
            let string: String = as_string(bytes, offset, length);
            offset += length;

            strings.push(string);
        }

        (offset, strings)
    }

    fn make_conditions(variables: Vec<u32>, values: Vec<u32>,
                       conditions: Vec<String>) -> Vec<Condition> {
        let mut ret_conditions: Vec<Condition> = Vec::with_capacity(variables.len());

        for i in 0..variables.len() {
            let operator: u8 = (variables[i] >> 24) as u8;
            let operator: Operator = Operator::new(operator);
            let variable: u32 = variables[i] & 0x00ffffff;

            let value: U32OrString = if *operator.is_value_variable() {
                U32OrString::U32(values[i])
            } else {
              U32OrString::String(conditions[i].clone())
            };

            ret_conditions.push(Condition::new(variable, operator, value));
        }

        ret_conditions
    }

    fn parse_cases(bytes: &[u8], case_count: usize, else_case: bool) -> (usize, u32, Vec<Case>) {
        let else_case: usize = if else_case { 1 } else { 0 };
        Case::parse_multiple(bytes, case_count + else_case)
    }
}
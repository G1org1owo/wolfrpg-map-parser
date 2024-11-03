mod options;
mod operation;
mod special_operation;

use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};
use crate::command::common::u32_or_string::U32OrString;
use crate::command::party_graphics_command::operation::Operation;
use crate::command::party_graphics_command::options::Options;

#[derive(Serialize)]
pub struct PartyGraphicsCommand {
    options: Options,
    member: Option<u32>,
    graphics: Option<U32OrString>,
}

impl PartyGraphicsCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let options: Options = Options::new(options);
        offset += 4;

        let member: Option<u32> = match *options.operation() {
            Operation::Remove | Operation::Insert | Operation::Replace => {
                let member: u32 = as_u32_le(&bytes[offset..offset + 4]);
                offset += 4;

                Some(member)
            }
            _ => None
        };

        let graphics_variable: Option<u32> = if options.is_graphics_variable() {
            let graphics_variable: u32 = as_u32_le(&bytes[offset..offset + 4]);
            offset += 4;

            Some(graphics_variable)
        } else {
            None
        };

        offset += 1; // Padding

        let is_graphics_string: bool = bytes[offset] != 0;
        offset += 1;

        let graphics_string: Option<String> = if is_graphics_string {
            let graphics_length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
            offset += 4;
            let graphics_string: String = as_string(bytes, offset, graphics_length);
            offset += graphics_length;

            Some(graphics_string)
        } else {
            None
        };

        let graphics: Option<U32OrString> = match (graphics_variable, graphics_string) {
            (Some(variable), None) => Some(U32OrString::U32(variable)),
            (None, Some(string)) => Some(U32OrString::String(string)),
            (None, None) => None,
            _ => unreachable!()
        };

        offset += 1; // Command end signature

        (offset, Self {
            options,
            member,
            graphics
        })
    }
}
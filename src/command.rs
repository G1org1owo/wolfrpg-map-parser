use std::fmt::format;
use serde::Serialize;
use crate::byte_utils::{as_u32_be};
use crate::show_choice_command::ShowChoiceCommand;
use crate::show_message_command::ShowMessageCommand;

const SHOW_MESSAGE_COMMAND: u32 = 0x01650000;
const SHOW_CHOICE_COMMAND: u32 = 0x02660000;
const EXIT_COMMAND: u32 = 0x01000000;

#[derive(Serialize)]
pub enum Command {
    ShowMessage(ShowMessageCommand),
    ShowChoice(ShowChoiceCommand),
    Exit(),
}

impl Command {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;
        let mut commands = 1;

        let command: Command = match as_u32_be(&bytes[offset..offset+4]) {
            SHOW_MESSAGE_COMMAND => { // Comment and Debug Text have similar codes
                offset+=4;
                let (bytes_read, command) : (usize, ShowMessageCommand)
                    = ShowMessageCommand::parse(&bytes[offset..]);
                offset += bytes_read;

                Ok(Command::ShowMessage(command))
            },
            SHOW_CHOICE_COMMAND => {
                offset+=4;
                let (bytes_read, commands_read, command) : (usize, u32, ShowChoiceCommand)
                    = ShowChoiceCommand::parse(&bytes[offset..]);
                offset += bytes_read;
                commands += commands_read;

                Ok(Command::ShowChoice(command))
            },
            EXIT_COMMAND => {
                offset+=4;
                offset+=4; // Not sure what the contents of the EXIT command are at the moment

                Ok(Command::Exit())
            }
            _ => Err("Unknown command code")
        }.expect(format(format_args!(
            "Could not interpret command 0x{:08x}",
            as_u32_be(&bytes[offset..offset+4])
        )).as_str());

        (offset, commands, command)
    }
}
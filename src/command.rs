use serde::Serialize;
use crate::byte_utils::{as_u32_be};
use crate::show_message_command::ShowMessageCommand;

#[derive(Serialize)]
pub enum Command {
    ShowMessage(ShowMessageCommand),
    ShowChoice(ShowChoiceCommand),
    Exit(),
}

impl Command {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;
        let command: Command = match as_u32_be(&bytes[offset..offset+4]) {
            0x01650000 => { // Comment and Debug Text have similar codes
                offset+=4;
                let (bytes_read, command) : (usize, ShowMessageCommand)
                    = ShowMessageCommand::parse(&bytes[offset..]);
                offset += bytes_read;

                Command::ShowMessage(command)
            },
            _ => {Command::Exit()}
        };

        (offset, command)
    }
}

#[derive(Serialize)]
struct ShowChoiceCommand {

}

impl ShowChoiceCommand {

}
use serde::Serialize;
use crate::byte_utils::{as_u32_be, as_u32_le};

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
struct ShowMessageCommand {
    message: String
}

impl ShowMessageCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 3;

        let message_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;

        let message: String = String::from_utf8(bytes[offset..offset+message_length-1].to_vec())
            .unwrap();

        (0, Self {
            message
        })
    }
}

#[derive(Serialize)]
struct ShowChoiceCommand {

}

impl ShowChoiceCommand {

}
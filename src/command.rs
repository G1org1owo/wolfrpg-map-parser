use std::fmt::format;
use serde::Serialize;
use crate::byte_utils::as_u32_be;
use show_choice_command::ShowChoiceCommand;
use show_message_command::ShowMessageCommand;
use crate::command::comment_command::CommentCommand;
use crate::command::db_management_command::DBManagementCommand;
use crate::command::debug_text_command::DebugTextCommand;
use crate::command::set_variable_command::SetVariableCommand;

mod show_choice_command;
mod show_message_command;
mod comment_command;
mod debug_text_command;
mod set_variable_command;
mod db_management_command;
mod common;

const SHOW_MESSAGE_COMMAND: u32         = 0x01650000;
const COMMENT_COMMAND: u32              = 0x01670000;
const DEBUG_TEXT_COMMAND: u32           = 0x016A0000;
const FORCE_CLOSE_MESSAGE_COMMAND: u32  = 0x01690000;
const CLEAR_DEBUG_TEXT_COMMAND: u32     = 0x016B0000;
const SHOW_CHOICE_COMMAND: u32          = 0x02660000;
const SET_VARIABLE_COMMAND_BASE: u32    = 0x05790000;
const SET_VARIABLE_COMMAND_RANGE: u32   = 0x06790000;
const DB_MANAGEMENT_BASE: u32           = 0x06fa0000;
const DB_MANAGEMENT_STRING: u32         = 0x05fa0000;
const DB_MANAGEMENT_CSV: u32            = 0x06fb0000;
const EXIT_COMMAND: u32                 = 0x01000000;

#[derive(Serialize)]
pub enum Command {
    ShowMessage(ShowMessageCommand),
    Comment(CommentCommand),
    DebugText(DebugTextCommand),
    ForceCloseMessage(),
    ClearDebugText(),
    ShowChoice(ShowChoiceCommand),
    SetVariable(SetVariableCommand),
    DBManagement(DBManagementCommand),
    Exit(),
}

impl Command {
    pub fn parse(bytes: &[u8]) -> (usize, u32, Self) {
        let mut offset: usize = 0;
        let mut commands = 1;

        let signature = as_u32_be(&bytes[offset..offset+4]);
        offset += 5;

        let command: Command = match signature {
            SHOW_MESSAGE_COMMAND => {
                let (bytes_read, command) : (usize, ShowMessageCommand)
                    = ShowMessageCommand::parse(&bytes[offset..]);
                offset += bytes_read;

                Ok(Command::ShowMessage(command))
            },

            COMMENT_COMMAND => {
                let (bytes_read, command): (usize, CommentCommand)
                    = CommentCommand::parse(&bytes[offset..]);
                offset += bytes_read;

                Ok(Command::Comment(command))
            },

            DEBUG_TEXT_COMMAND => {
                let (bytes_read, command): (usize,DebugTextCommand)
                    = DebugTextCommand::parse(&bytes[offset..]);
                offset += bytes_read;

                Ok(Command::DebugText(command))
            }

            FORCE_CLOSE_MESSAGE_COMMAND => {
                offset += 3;

                Ok(Command::ForceCloseMessage())
            }

            CLEAR_DEBUG_TEXT_COMMAND => {
                offset += 3;

                Ok(Command::ClearDebugText())
            }

            SHOW_CHOICE_COMMAND => {
                let (bytes_read, commands_read, command) : (usize, u32, ShowChoiceCommand)
                    = ShowChoiceCommand::parse(&bytes[offset..]);
                offset += bytes_read;
                commands += commands_read;

                Ok(Command::ShowChoice(command))
            },

            SET_VARIABLE_COMMAND_BASE => {
                let (bytes_read, command): (usize, SetVariableCommand)
                    = SetVariableCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SetVariable(command))
            }

            SET_VARIABLE_COMMAND_RANGE => {
                let (bytes_read, command): (usize, SetVariableCommand)
                    = SetVariableCommand::parse_range(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SetVariable(command))
            }

            DB_MANAGEMENT_BASE => {
                let (bytes_read, command): (usize, DBManagementCommand)
                    = DBManagementCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::DBManagement(command))
            }

            EXIT_COMMAND => {
                offset+=3; // Not sure what the contents of the EXIT command are at the moment

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
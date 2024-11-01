use serde::Serialize;
use crate::byte_utils::as_u32_be;
use show_choice_command::ShowChoiceCommand;
use show_message_command::ShowMessageCommand;
use crate::command::comment_command::CommentCommand;
use crate::command::db_management_command::DBManagementCommand;
use crate::command::debug_text_command::DebugTextCommand;
use crate::command::effect_command::EffectCommand;
use crate::command::input_key_command::InputKeyCommand;
use crate::command::number_condition_command::NumberConditionCommand;
use crate::command::picture_command::PictureCommand;
use crate::command::save_load_command::SaveLoadCommand;
use crate::command::set_string_command::SetStringCommand;
use crate::command::set_variable_command::SetVariableCommand;
use crate::command::set_variable_plus_command::SetVariablePlusCommand;
use crate::command::sound_command::SoundCommand;
use crate::command::string_condition_command::StringConditionCommand;

mod show_choice_command;
mod show_message_command;
mod comment_command;
mod debug_text_command;
mod set_variable_command;
mod db_management_command;
mod common;
mod set_string_command;
mod set_variable_plus_command;
mod number_condition_command;
mod string_condition_command;
mod input_key_command;
mod picture_command;
mod effect_command;
mod sound_command;
mod save_load_command;

const SHOW_MESSAGE_COMMAND: u32                 = 0x01650000;
const COMMENT_COMMAND: u32                      = 0x01670000;
const DEBUG_TEXT_COMMAND: u32                   = 0x016A0000;
const FORCE_CLOSE_MESSAGE_COMMAND: u32          = 0x01690000;
const CLEAR_DEBUG_TEXT_COMMAND: u32             = 0x016B0000;
const SHOW_CHOICE_COMMAND: u32                  = 0x02660000;
const SET_VARIABLE_COMMAND_BASE: u32            = 0x05790000;
const SET_VARIABLE_COMMAND_RANGE: u32           = 0x06790000;
const DB_MANAGEMENT_COMMAND_BASE: u32           = 0x06fa0000;
const DB_MANAGEMENT_COMMAND_STRING: u32         = 0x05fa0000;
const DB_MANAGEMENT_COMMAND_CSV: u32            = 0x06fb0000;
const SET_STRING_COMMAND_BASE: u32              = 0x037a0000;
const SET_STRING_COMMAND_DYNAMIC: u32           = 0x047a0000;
const SET_VARIABLE_PLUS_COMMAND_BASE: u32       = 0x057c0000;
const SET_VARIABLE_PLUS_COMMAND_OTHER: u32      = 0x047c0000;
const NUMBER_CONDITION_COMMAND: u32             = 0x056f0000;
const NUMBER_CONDITION_COMMAND_DOUBLE: u32      = 0x086f0000;
const NUMBER_CONDITION_COMMAND_TRIPLE: u32      = 0x0b6f0000;
const STRING_CONDITION_COMMAND: u32             = 0x03700000;
const STRING_CONDITION_COMMAND_TWO: u32         = 0x04700000;
const STRING_CONDITION_COMMAND_THREE: u32       = 0x05700000;
const STRING_CONDITION_COMMAND_FOUR: u32        = 0x06700000;
const STRING_CONDITION_COMMAND_FIVE: u32        = 0x07700000;
const STRING_CONDITION_COMMAND_SIX: u32         = 0x08700000;
const STRING_CONDITION_COMMAND_SEVEN: u32       = 0x09700000;
const STRING_CONDITION_COMMAND_EIGHT: u32       = 0x0a700000;
const INPUT_KEY_COMMAND_BASE: u32               = 0x037b0000;
const INPUT_KEY_COMMAND_KEYBOARD_OR_PAD: u32    = 0x047b0000;
const AUTOMATIC_INPUT_COMMAND_BASIC: u32        = 0x027d0000;
const AUTOMATIC_INPUT_COMMAND_KEYBOARD: u32     = 0x037d0000;
const AUTOMATIC_INPUT_COMMAND_MOUSE: u32        = 0x047d0000;
const INPUT_TOGGLE_COMMAND_BASIC: u32           = 0x027e0000;
const INPUT_TOGGLE_COMMAND_DEVICE: u32          = 0x037e0000;
const PICTURE_SHOW_COMMAND_BASE: u32            = 0x0c960000;
const PICTURE_SHOW_COMMAND_BASE_BY_VAR: u32     = 0x0d960000;
const PICTURE_SHOW_COMMAND_COLORS: u32          = 0x0e960000;
const PICTURE_SHOW_COMMAND_DELAY: u32           = 0x0f960000;
const PICTURE_SHOW_COMMAND_RANGE: u32           = 0x10960000;
const PICTURE_SHOW_COMMAND_COLOR_VALUES: u32    = 0x13960000;
const PICTURE_SHOW_COMMAND_ZOOM: u32            = 0x14960000;
const PICTURE_SHOW_COMMAND_FREE_TRANSFORM: u32  = 0x1a960000;
const PICTURE_ERASE_COMMAND_DELAY_RESET: u32    = 0x03960000;
const PICTURE_ERASE_COMMAND_BASE: u32           = 0x04960000;
const PICTURE_ERASE_COMMAND_DELAY: u32          = 0x05960000;
const PICTURE_ERASE_COMMAND_RANGE: u32          = 0x07960000;
const EFFECT_COMMAND_BASE: u32                  = 0x08220100;
const EFFECT_COMMAND_MAP_SHAKE: u32             = 0x03180100;
const EFFECT_COMMAND_SCROLL_SCREEN: u32         = 0x04190100;
const EFFECT_COMMAND_CHANGE_COLOR: u32          = 0x03970000;
const SOUND_COMMAND_FILENAME: u32               = 0x088c0000;
const SOUND_COMMAND_FILENAME_SE: u32            = 0x078c0000;
const SOUND_COMMAND_VARIABLE: u32               = 0x058c0000;
const SOUND_COMMAND_FREE_ALL: u32               = 0x028c0000;
const SOUND_COMMAND_FREE_ALL_VARIABLE: u32      = 0x048c0000;
const SAVE_LOAD_COMMAND_BASE: u32               = 0x03dc0000;
const SAVE_LOAD_COMMAND_LOAD_VARIABLE: u32      = 0x05dd0000;
const SAVE_LOAD_COMMAND_SAVE_VARIABLE: u32      = 0x05de0000;
const EXIT_COMMAND: u32                         = 0x01000000;

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
    SetString(SetStringCommand),
    SetVariablePlus(SetVariablePlusCommand),
    NumberConditionCommand(NumberConditionCommand),
    StringConditionCommand(StringConditionCommand),
    InputKeyCommand(InputKeyCommand),
    PictureCommand(PictureCommand),
    EffectCommand(EffectCommand),
    SoundCommand(SoundCommand),
    SaveLoadCommand(SaveLoadCommand),
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

            DB_MANAGEMENT_COMMAND_BASE => {
                let (bytes_read, command): (usize, DBManagementCommand)
                    = DBManagementCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::DBManagement(command))
            }

            DB_MANAGEMENT_COMMAND_STRING => {
                let (bytes_read, command): (usize, DBManagementCommand)
                    = DBManagementCommand::parse_string(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::DBManagement(command))
            }

            DB_MANAGEMENT_COMMAND_CSV => {
                let (bytes_read, command): (usize, DBManagementCommand)
                    = DBManagementCommand::parse_csv(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::DBManagement(command))
            }

            SET_STRING_COMMAND_BASE => {
                let (bytes_read, command): (usize, SetStringCommand)
                    = SetStringCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SetString(command))
            }

            SET_STRING_COMMAND_DYNAMIC => {
                let (bytes_read, command): (usize, SetStringCommand)
                    = SetStringCommand::parse_dynamic(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SetString(command))
            }

            SET_VARIABLE_PLUS_COMMAND_BASE => {
                let (bytes_read, command): (usize, SetVariablePlusCommand)
                    = SetVariablePlusCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SetVariablePlus(command))
            }

            SET_VARIABLE_PLUS_COMMAND_OTHER => {
                let (bytes_read, command): (usize, SetVariablePlusCommand)
                    = SetVariablePlusCommand::parse_other(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SetVariablePlus(command))
            }

            NUMBER_CONDITION_COMMAND | NUMBER_CONDITION_COMMAND_DOUBLE |
            NUMBER_CONDITION_COMMAND_TRIPLE => {
                let (bytes_read, commands_read, command): (usize, u32, NumberConditionCommand)
                    = NumberConditionCommand::parse(&bytes[offset..]);

                offset += bytes_read;
                commands += commands_read;

                Ok(Command::NumberConditionCommand(command))
            }

            STRING_CONDITION_COMMAND | STRING_CONDITION_COMMAND_TWO |
            STRING_CONDITION_COMMAND_THREE | STRING_CONDITION_COMMAND_FOUR |
            STRING_CONDITION_COMMAND_FIVE | STRING_CONDITION_COMMAND_SIX |
            STRING_CONDITION_COMMAND_SEVEN | STRING_CONDITION_COMMAND_EIGHT => {
                let (bytes_read, commands_read, command) : (usize, u32, StringConditionCommand)
                    = StringConditionCommand::parse(&bytes[offset..], signature);

                offset += bytes_read;
                commands += commands_read;

                Ok(Command::StringConditionCommand(command))
            }

            INPUT_KEY_COMMAND_BASE => {
                let (bytes_read, command): (usize, InputKeyCommand)
                    = InputKeyCommand::parse_input_key_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::InputKeyCommand(command))
            }

            INPUT_KEY_COMMAND_KEYBOARD_OR_PAD => {
                let (bytes_read, command): (usize, InputKeyCommand)
                    = InputKeyCommand::parse_input_key_keyboard_or_pad(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::InputKeyCommand(command))
            }

            AUTOMATIC_INPUT_COMMAND_BASIC | AUTOMATIC_INPUT_COMMAND_MOUSE => {
                let (bytes_read, command): (usize, InputKeyCommand)
                    = InputKeyCommand::parse_automatic_input_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::InputKeyCommand(command))
            }

            AUTOMATIC_INPUT_COMMAND_KEYBOARD => {
                let (bytes_read, command): (usize, InputKeyCommand)
                    = InputKeyCommand::parse_automatic_input_keyboard(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::InputKeyCommand(command))
            }

            INPUT_TOGGLE_COMMAND_BASIC | INPUT_TOGGLE_COMMAND_DEVICE => {
                let (bytes_read, command): (usize, InputKeyCommand)
                    = InputKeyCommand::parse_input_toggle(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::InputKeyCommand(command))
            }

            PICTURE_SHOW_COMMAND_BASE | PICTURE_SHOW_COMMAND_BASE_BY_VAR => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_show_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_SHOW_COMMAND_COLORS => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_show_colors(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_SHOW_COMMAND_DELAY => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_show_delay(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_SHOW_COMMAND_RANGE => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_show_range(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_SHOW_COMMAND_COLOR_VALUES => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_color_values(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_SHOW_COMMAND_ZOOM => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_show_zoom(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_SHOW_COMMAND_FREE_TRANSFORM => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_show_free_transform(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_ERASE_COMMAND_DELAY_RESET => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_erase_delay_reset(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_ERASE_COMMAND_BASE => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_erase_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_ERASE_COMMAND_DELAY => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_erase_delay(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            PICTURE_ERASE_COMMAND_RANGE => {
                let (bytes_read, command): (usize, PictureCommand)
                    = PictureCommand::parse_erase_range(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::PictureCommand(command))
            }

            EFFECT_COMMAND_BASE => {
                let (bytes_read, command): (usize, EffectCommand)
                    = EffectCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::EffectCommand(command))
            }

            EFFECT_COMMAND_MAP_SHAKE => {
                let (bytes_read, command): (usize, EffectCommand)
                    = EffectCommand::parse_map_shake(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::EffectCommand(command))
            }

            EFFECT_COMMAND_SCROLL_SCREEN => {
                let (bytes_read, command): (usize, EffectCommand)
                    = EffectCommand::parse_scroll_screen(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::EffectCommand(command))
            }

            EFFECT_COMMAND_CHANGE_COLOR => {
                let (bytes_read, command): (usize, EffectCommand)
                    = EffectCommand::parse_change_color(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::EffectCommand(command))
            }

            SOUND_COMMAND_FILENAME | SOUND_COMMAND_FILENAME_SE => {
                let (bytes_read, command): (usize, SoundCommand)
                    = SoundCommand::parse_filename(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SoundCommand(command))
            }

            SOUND_COMMAND_VARIABLE => {
                let (bytes_read, command): (usize, SoundCommand)
                    = SoundCommand::parse_variable(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SoundCommand(command))
            }

            SOUND_COMMAND_FREE_ALL | SOUND_COMMAND_FREE_ALL_VARIABLE => {
                let (bytes_read, command): (usize, SoundCommand)
                    = SoundCommand::parse_free_all(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SoundCommand(command))
            }

            SAVE_LOAD_COMMAND_BASE => {
                let (bytes_read, command): (usize, SaveLoadCommand)
                    = SaveLoadCommand::parse_base(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SaveLoadCommand(command))
            }

            SAVE_LOAD_COMMAND_LOAD_VARIABLE => {
                let (bytes_read, command): (usize, SaveLoadCommand)
                    = SaveLoadCommand::parse_load_variable(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SaveLoadCommand(command))
            }

            SAVE_LOAD_COMMAND_SAVE_VARIABLE => {
                let (bytes_read, command): (usize, SaveLoadCommand)
                    = SaveLoadCommand::parse_save_variable(&bytes[offset..]);

                offset += bytes_read;

                Ok(Command::SaveLoadCommand(command))
            }

            EXIT_COMMAND => {
                offset+=3; // Not sure what the contents of the EXIT command are at the moment

                Ok(Command::Exit())
            }
            _ => Err("Unknown command code")
        }.expect(format!("Could not interpret command 0x{:08x}", signature)
            .as_str());

        (offset, commands, command)
    }
}